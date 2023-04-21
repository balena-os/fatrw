use anyhow::{anyhow, Context, Result};
use log::debug;
use path_absolutize::Absolutize;

use alloc::borrow::Cow;
use std::fs::{metadata, File, OpenOptions};
use std::io::Result as IOResult;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub use std::fs::{copy, read, remove_file, rename};

pub fn as_absolute(path: &Path) -> Result<Cow<'_, Path>> {
    path.absolutize()
        .context(format!("Failed to absolutize {}", path.display()))
}

pub fn get_file_name(path: &Path) -> Result<String> {
    let file_name_os = path
        .file_name()
        .ok_or_else(|| anyhow!("No file name in path {}", path.display()))?;

    let file_name = file_name_os
        .to_str()
        .ok_or_else(|| anyhow!("File name is not a valid UTF-8 string {:?}", file_name_os))?;

    Ok(file_name.to_owned())
}

pub fn get_file_mode(path: &Path) -> Result<u32> {
    let meta = metadata(path)?;
    let perm = meta.permissions();
    Ok(perm.mode())
}

pub fn get_parent_as_string(path: &Path) -> Result<String> {
    let parent_path = path
        .parent()
        .ok_or_else(|| anyhow!("No parent in path {}", path.display()))?;

    let parent = parent_path.to_str().ok_or_else(|| {
        anyhow!(
            "Parent is not a valid UTF-8 string {}",
            parent_path.display()
        )
    })?;

    Ok(parent.to_owned())
}

pub fn fsync_parent_dir(path: &Path) -> Result<()> {
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

pub fn create_file(path: &Path, mode: Option<u32>, content: &[u8]) -> IOResult<()> {
    if let Err(e) = create_file_unclean(path, mode, content) {
        // Remove any left over file content in case of failure
        remove_file(path).ok();
        return Err(e);
    }

    Ok(())
}

fn create_file_unclean(path: &Path, mode: Option<u32>, content: &[u8]) -> IOResult<()> {
    let mut file = open_with_mode(path, mode)?;

    file.write_all(content)?;
    file.sync_all()?;

    debug!("Created {}", file_name_display(path));

    Ok(())
}

pub fn clean_copy(from: &Path, to: &Path) -> IOResult<u64> {
    match copy(from, to) {
        Ok(res) => Ok(res),
        Err(err) => {
            // Remove any left over file content in case of failure
            remove_file(to).ok();
            Err(err)
        }
    }
}

fn open_with_mode(path: &Path, mode: Option<u32>) -> IOResult<File> {
    let mut open_options = OpenOptions::new();

    open_options.create(true).write(true);

    if let Some(octal_mode) = mode {
        open_options.mode(octal_mode);
    }

    open_options.open(path)
}

pub fn file_name_display(path: &Path) -> Cow<'_, str> {
    path.file_name().unwrap_or_default().to_string_lossy()
}

pub fn is_storage_full_error(err: &std::io::Error) -> bool {
    // TODO: Use io::ErrorKind::StorageFull when stabilized
    err.raw_os_error() == Some(28_i32)
}
