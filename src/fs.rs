use anyhow::{anyhow, Context, Result};
use log::debug;
use path_absolutize::Absolutize;

use std::borrow::Cow;
use std::fs::{copy, metadata, read, remove_file, rename, File, OpenOptions};
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use crate::checksum::{extract_checksum_from_path, md5sum};

pub(crate) fn as_absolute(path: &Path) -> Result<Cow<'_, Path>> {
    path.absolutize()
        .context(format!("Failed to absolutize {}", path.display()))
}

pub(crate) fn get_file_name(path: &Path) -> Result<String> {
    let file_name_os = path
        .file_name()
        .ok_or_else(|| anyhow!("No file name in path {}", path.display()))?;

    let file_name = file_name_os
        .to_str()
        .ok_or_else(|| anyhow!("File name is not a valid UTF-8 string {:?}", file_name_os))?;

    Ok(file_name.to_string())
}

pub(crate) fn get_file_mode(path: &Path) -> Result<u32> {
    let meta = metadata(path)?;
    let perm = meta.permissions();
    Ok(perm.mode())
}

pub(crate) fn get_parent_as_string(path: &Path) -> Result<String> {
    let parent_path = path
        .parent()
        .ok_or_else(|| anyhow!("No parent in path {}", path.display()))?;

    let parent = parent_path.to_str().ok_or_else(|| {
        anyhow!(
            "Parent is not a valid UTF-8 string {}",
            parent_path.display()
        )
    })?;

    Ok(parent.to_string())
}

pub(crate) fn commit_md5sum_file(md5sum_path: &Path, path: &Path) -> Result<Vec<u8>> {
    debug!("Committing checksum file");

    let content = verify_checksum(md5sum_path)?;

    let temp_path = md5sum_path.with_extension("tmp");

    copy(&md5sum_path, &temp_path).context(format!(
        "Failed to copy to a temporary location {} -> {}",
        md5sum_path.display(),
        temp_path.display()
    ))?;
    debug!(
        "Copied {} {}",
        file_name_display(md5sum_path),
        file_name_display(&temp_path)
    );

    rename(&temp_path, &path).context(format!(
        "Failed to rename temporary file to target file {} -> {}",
        temp_path.display(),
        path.display()
    ))?;
    debug!(
        "Renamed {} {}",
        file_name_display(&temp_path),
        file_name_display(path)
    );

    fsync_parent_dir(path)?;

    remove_file(&md5sum_path).context(format!("Failed to remove {}", md5sum_path.display()))?;
    debug!("Removed {}", file_name_display(md5sum_path));

    fsync_parent_dir(path)?;

    Ok(content)
}

pub(crate) fn verify_checksum(path: &Path) -> Result<Vec<u8>> {
    let content =
        read(&path).context(format!("Failed to read checksum file {}", path.display()))?;

    let content_checksum = md5sum(&content);
    let file_name_checksum = extract_checksum_from_path(path)?;

    if content_checksum != file_name_checksum {
        Err(anyhow!(
            "Content and file name checksums do not match {} != {}",
            content_checksum,
            file_name_checksum
        ))
    } else {
        debug!("Checksum verified");
        Ok(content)
    }
}

pub(crate) fn fsync_parent_dir(path: &Path) -> Result<()> {
    let parent_dir = path
        .parent()
        .ok_or_else(|| anyhow!("Cannot evaluate the parent directory of {}", path.display()))?;

    let f = File::open(parent_dir)
        .context(format!("Failed to open directory {}", parent_dir.display()))?;
    f.sync_all()
        .context(format!("Failed to sync directory {}", parent_dir.display()))?;
    debug!("Dir fsynced {}", parent_dir.display());

    Ok(())
}

pub(crate) fn create_file(path: &Path, mode: Option<u32>, content: &[u8]) -> Result<()> {
    let mut file = open_with_mode(path, mode)?;

    file.write_all(content)?;
    file.sync_all()?;

    debug!("Created {}", file_name_display(path));

    Ok(())
}

fn open_with_mode(path: &Path, mode: Option<u32>) -> Result<File> {
    let mut open_options = OpenOptions::new();

    open_options.create(true).write(true);

    if let Some(octal_mode) = mode {
        open_options.mode(octal_mode);
    }

    Ok(open_options.open(path)?)
}

pub(crate) fn file_name_display(path: &Path) -> Cow<'_, str> {
    path.file_name().unwrap_or_default().to_string_lossy()
}
