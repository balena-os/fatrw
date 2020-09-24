use anyhow::{anyhow, Context, Result};
use path_absolutize::Absolutize;

use std::fs::{copy, read_to_string, remove_file, rename, File, OpenOptions};
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

use crate::checksum::{extract_checksum_from_path, md5sum};

pub fn as_absolute<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    path.as_ref()
        .absolutize()
        .context(format!("Failed to absolutize {:?}", path.as_ref()))
        .map(|p| p.into())
}

pub fn get_file_name<P: AsRef<Path>>(path: P) -> Result<String> {
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

pub fn get_parent_as_string<P: AsRef<Path>>(path: P) -> Result<String> {
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

pub fn commit_md5sum_file<P: AsRef<Path>, Q: AsRef<Path>>(
    md5sum_path: P,
    path: Q,
) -> Result<String> {
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

pub fn verify_checksum<P: AsRef<Path>>(path: P) -> Result<String> {
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

pub fn fsync_parent_dir<P: AsRef<Path>>(path: P) -> Result<()> {
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

pub fn mode_from_string(mode: Option<&str>) -> Result<Option<u32>> {
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

pub fn create_file<P: AsRef<Path>>(path: P, mode: Option<u32>, content: &str) -> Result<()> {
    let mut file = open_with_mode(&path, mode)?;

    file.write_all(content.as_bytes())?;
    file.sync_all()?;

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
