use anyhow::{anyhow, Context, Result};
use clap::Clap;
use getrandom::getrandom;
use glob::glob;
use md5::{Digest, Md5};
use path_absolutize::Absolutize;
use regex::Regex;

use std::{
    fs::{copy, read_to_string, remove_file, rename, File, OpenOptions},
    io::Write,
    os::unix::fs::OpenOptionsExt,
    path::{Path, PathBuf},
    process,
};

#[derive(Clap, Debug)]
#[clap()]
struct Opts {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Clap, Debug)]
#[clap()]
enum Command {
    #[clap(alias = "w")]
    Write(WriteArgs),
    #[clap(alias = "r")]
    Read(ReadArgs),
}

#[derive(Clap, Debug)]
#[clap()]
struct WriteArgs {
    path: PathBuf,
    content: String,
    mode: Option<String>,
}

#[derive(Clap, Debug)]
#[clap()]
struct ReadArgs {
    path: PathBuf,
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    match opts.command {
        Command::Write(write_args) => execute_write(write_args),
        Command::Read(read_args) => execute_read(read_args),
    }
}

fn execute_write(write_args: WriteArgs) -> Result<()> {
    println!("Path: {:?}", write_args.path);

    let mode = mode_from_string(write_args.mode.as_deref())?;

    let path = as_absolute(&write_args.path)?;
    println!("Absolute path: {:?}", path);

    let checksum = md5sum(&write_args.content);
    println!("MD5 checksum: {}", checksum);

    let md5sum_path = generate_md5sum_path(&path, &checksum)?;

    create_file(&md5sum_path, mode, &write_args.content)
        .context(format!("Failed to create checksum file {:?}", md5sum_path))?;
    println!("Created {:?}", md5sum_path.file_name().unwrap());

    fsync_parent_dir(&path)?;

    commit_md5sum_file(md5sum_path, &path)?;

    Ok(())
}

fn open_with_mode<P: AsRef<Path>>(path: P, mode: Option<u32>) -> Result<File> {
    let mut open_options = OpenOptions::new();

    open_options.create(true).write(true);

    if let Some(octal_mode) = mode {
        open_options.mode(octal_mode);
    }

    Ok(open_options.open(path)?)
}

fn create_file<P: AsRef<Path>>(path: P, mode: Option<u32>, content: &str) -> Result<()> {
    let mut file = open_with_mode(&path, mode)?;

    file.write_all(content.as_bytes())?;
    file.sync_all()?;

    Ok(())
}

fn mode_from_string(mode: Option<&str>) -> Result<Option<u32>> {
    Ok(if let Some(ref octal_str) = mode {
        let octal_mode = parse_file_mode(octal_str)?;
        println!("File mode: {:o}", octal_mode);
        Some(octal_mode)
    } else {
        None
    })
}

fn parse_file_mode(octal_str: &str) -> Result<u32> {
    u32::from_str_radix(octal_str, 8).context("Parsing file mode failed")
}

fn fsync_parent_dir<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();
    let parent_dir = path.parent().context(format!(
        "Cannot evaluate the parent directory of {:?}",
        path
    ))?;

    let f = File::open(parent_dir).context(format!("Failed to open directory {:?}", parent_dir))?;
    f.sync_all()
        .context(format!("Failed to sync directory {:?}", parent_dir))?;
    println!("Parent dir synced {:?}", parent_dir);

    Ok(())
}

fn verify_checksum<P: AsRef<Path>>(path: P) -> Result<String> {
    let path = path.as_ref();
    let content =
        read_to_string(&path).context(format!("Failed to read checksum file {:?}", path))?;

    let content_checksum = md5sum(&content);
    let file_name_checksum = extract_checksum_from_path(path)?;

    if content_checksum != file_name_checksum {
        Err(anyhow!(
            "Content and file name checksums do not match {} != {}",
            content_checksum,
            file_name_checksum
        ))
    } else {
        println!("Checksum verified");
        Ok(content)
    }
}

fn extract_checksum_from_path<P: AsRef<Path>>(path: P) -> Result<String> {
    let filename_re = Regex::new(r"^\..*\.(?P<hash>[0-9a-f]{32}).md5sum$").unwrap();

    let file_name = get_file_name(path)?;

    filename_re
        .captures(&file_name)
        .and_then(|cap| cap.name("hash").map(|hash| hash.as_str().to_string()))
        .context("Cannot capture MD5 checksum from file name")
}

fn get_file_name<P: AsRef<Path>>(path: P) -> Result<String> {
    let path = path.as_ref();
    let file_name_os = path
        .file_name()
        .context(format!("No file name in path {:?}", path))?;

    let file_name = file_name_os.to_str().context(format!(
        "File name is not a valid UTF-8 string {:?}",
        file_name_os
    ))?;

    Ok(file_name.to_string())
}

fn get_parent_as_string<P: AsRef<Path>>(path: P) -> Result<String> {
    let path = path.as_ref();
    let parent_os = path
        .parent()
        .context(format!("No parent in path {:?}", path))?;

    let parent = parent_os.to_str().context(format!(
        "Parent is not a valid UTF-8 string {:?}",
        parent_os
    ))?;

    Ok(parent.to_string())
}

fn md5sum(content: &str) -> String {
    format!("{:x}", Md5::digest(content.as_bytes()))
}

fn generate_md5sum_path<P: AsRef<Path>>(path: P, checksum: &str) -> Result<PathBuf> {
    let path = path.as_ref();

    let random_suffix = generate_random_string();

    let target_name = get_file_name(&path)?;

    let md5sum_name = format!(".{}.{}.{}.md5sum", target_name, random_suffix, checksum);
    println!("Checksum file name: {}", md5sum_name);

    let md5sum_path = path.with_file_name(md5sum_name);

    Ok(md5sum_path)
}

fn generate_random_string() -> String {
    let mut string = String::new();
    let buf = generate_random_buf();
    for num in &buf {
        string.push_str(&format!("{:02x}", num));
    }
    string
}

fn generate_random_buf() -> [u8; 4] {
    let mut buf = [0u8; 4];
    if let Ok(()) = getrandom(&mut buf) {
        buf
    } else {
        let process_bytes = process::id().to_be_bytes();
        buf[..4].clone_from_slice(&process_bytes);
        buf
    }
}

fn as_absolute<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    path.as_ref()
        .absolutize()
        .context(format!("Failed to absolutize {:?}", path.as_ref()))
        .map(|p| p.into())
}

fn execute_read(read_args: ReadArgs) -> Result<()> {
    println!("Path: {:?}", read_args.path);

    let path = as_absolute(&read_args.path)?;
    println!("Absolute path: {:?}", path);

    let content = if let Some(content) = process_md5sums(&path) {
        content
    } else {
        read_to_string(&path).context(format!("Failed to read target file {:?}", path))?
    };

    println!("Content: {}", content);

    Ok(())
}

fn process_md5sums<P: AsRef<Path>>(path: P) -> Option<String> {
    let file_name = get_file_name(&path).ok()?;
    let parent = get_parent_as_string(&path).ok()?;
    println!("Parent: {}", parent);

    let pattern = format!("{}/.{}.*.*.md5sum", parent, file_name);
    println!("Pattern: {}", pattern);

    let mut content = None;

    for entry in glob(&pattern).context("Failed to read md5sum glob pattern").ok()? {
        match entry {
            Ok(md5sum_path) => {
                if content == None {
                    println!("Found .md5sum file: {}", md5sum_path.display());
                    if let Ok(md5sum_content) = commit_md5sum_file(&md5sum_path, &path) {
                        content = Some(md5sum_content);
                    }
                }

                let temp_path = md5sum_path.with_extension("tmp");

                remove_file(&md5sum_path).ok();
                remove_file(&temp_path).ok();
            }
            Err(e) => println!("{:?}", e),
        }
    }

    content
}

fn commit_md5sum_file<P: AsRef<Path>, Q: AsRef<Path>>(md5sum_path: P, path: Q) -> Result<String> {
    let md5sum_path = md5sum_path.as_ref();
    let path = path.as_ref();

    let content = verify_checksum(&md5sum_path)?;

    let temp_path = md5sum_path.with_extension("tmp");

    copy(&md5sum_path, &temp_path).context(format!(
        "Failed to copy to a temporary location {:?} -> {:?}",
        md5sum_path, temp_path
    ))?;
    println!(
        "Copied {:?} -> {:?}",
        md5sum_path.file_name().unwrap(),
        temp_path.file_name().unwrap()
    );

    rename(&temp_path, &path).context(format!(
        "Failed to rename temporary file to target file {:?} -> {:?}",
        temp_path, path
    ))?;
    println!(
        "Renamed {:?} -> {:?}",
        temp_path.file_name().unwrap(),
        path.file_name().unwrap()
    );

    remove_file(&md5sum_path).context(format!("Failed to remove {:?}", md5sum_path))?;
    println!("Removed {:?}", md5sum_path.file_name().unwrap());

    fsync_parent_dir(&path)?;

    Ok(content)
}
