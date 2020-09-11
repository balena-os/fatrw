use anyhow::{anyhow, Context, Result};
use clap::Clap;
use getrandom::getrandom;
use md5::{Digest, Md5};
use path_absolutize::Absolutize;
use regex::Regex;

use std::{
    env::temp_dir,
    fs::{copy, read_to_string, File, OpenOptions},
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
}

#[derive(Clap, Debug)]
#[clap()]
struct WriteArgs {
    path: PathBuf,
    content: String,
    mode: Option<String>,
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    match opts.command {
        Command::Write(write_args) => execute_write(write_args),
    }
}

fn execute_write(write_args: WriteArgs) -> Result<()> {
    println!("Path: {:?}", write_args.path);
    println!("Content: {}", write_args.content);

    let mode = mode_from_string(write_args.mode.as_deref())?;

    let abs_path = write_args
        .path
        .absolutize()
        .context(format!("Failed to absolutize {:?}", write_args.path))?;

    create_hashed(&abs_path, mode, &write_args.content)?;

    Ok(())
}

fn create_hashed<P: AsRef<Path>>(path: P, mode: Option<u32>, content: &str) -> Result<()> {
    let hash = md5sum(content);
    println!("MD5 sum: {}", hash);

    let file_name = get_file_name(&path)?;

    let md5sum_file_name = format!("{}.{}.md5sum", file_name, hash);

    println!("Checksum file name: {}", md5sum_file_name);

    let md5sum_path = path.as_ref().with_file_name(md5sum_file_name);

    create_and_sync_file(&md5sum_path, mode, content)
        .context(format!("Failed to create checksum file {:?}", md5sum_path))?;

    verify_checksum(&md5sum_path)?;

    let temp_file_path = generate_temp_file_path(&file_name);

    copy(&md5sum_path, &temp_file_path).context("Failed to copy to a temporary location")?;

    fsync_parent_dir(temp_file_path)?;

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

fn create_and_sync_file<P: AsRef<Path>>(path: P, mode: Option<u32>, content: &str) -> Result<()> {
    create_file(&path, mode, content)?;

    fsync_parent_dir(&path)?;

    Ok(())
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
    let parent_dir = path.as_ref().parent().context(format!(
        "Cannot evaluate the parent directory of {:?}",
        path.as_ref()
    ))?;

    let f = File::open(parent_dir).context(format!("Failed to open directory {:?}", parent_dir))?;
    f.sync_all()
        .context(format!("Failed to sync directory {:?}", parent_dir))?;

    Ok(())
}

fn verify_checksum<P: AsRef<Path>>(path: P) -> Result<()> {
    let content = read_to_string(&path)
        .context(format!("Failed to read checksum file {:?}", path.as_ref()))?;

    let content_hash = md5sum(&content);
    let file_name_hash = extract_checksum_from_path(path)?;

    if content_hash != file_name_hash {
        Err(anyhow!(
            "Content and file name checksums do not match {} != {}",
            content_hash,
            file_name_hash
        ))
    } else {
        Ok(())
    }
}

fn extract_checksum_from_path<P: AsRef<Path>>(path: P) -> Result<String> {
    let filename_re = Regex::new(r"^.*\.(?P<hash>[0-9a-f]{32}).md5sum$").unwrap();

    let file_name = get_file_name(path)?;

    filename_re
        .captures(&file_name)
        .and_then(|cap| cap.name("hash").map(|hash| hash.as_str().to_string()))
        .context("Cannot capture MD5 checksum from file name")
}

fn get_file_name<P: AsRef<Path>>(path: P) -> Result<String> {
    let file_name_os = path
        .as_ref()
        .file_name()
        .context(format!("No file name in path {:?}", path.as_ref()))?;

    let file_name = file_name_os.to_str().context(format!(
        "File name is not a valid UTF-8 string {:?}",
        file_name_os
    ))?;

    Ok(file_name.to_string())
}

fn md5sum(content: &str) -> String {
    format!("{:x}", Md5::digest(content.as_bytes()))
}

fn generate_temp_file_path(file_name: &str) -> PathBuf {
    let mut path = temp_dir();
    path.push(format!("{}.{}", file_name, get_random_string()));
    path
}

fn get_random_string() -> String {
    let mut string = String::new();
    let buf = get_random_buf();
    for i in 0..buf.len() {
        string.push_str(&format!("{:02x}", buf[i]));
    }
    string
}

fn get_random_buf() -> [u8; 16] {
    let mut buf = [0u8; 16];
    if let Ok(()) = getrandom(&mut buf) {
        buf
    } else {
        let process_bytes = process::id().to_be_bytes();
        buf[..4].clone_from_slice(&process_bytes);
        buf
    }
}
