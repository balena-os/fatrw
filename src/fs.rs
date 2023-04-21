use anyhow::{anyhow, Context, Result};
use log::debug;

use std::fs::{metadata, File, OpenOptions};
use std::io::Result as IOResult;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub use std::fs::{copy, read, remove_file, rename};

use crate::path::file_name_display;

pub fn get_file_mode(path: &Path) -> Result<u32> {
    let meta = metadata(path)?;
    let perm = meta.permissions();
    Ok(perm.mode())
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

pub fn is_storage_full_error(err: &std::io::Error) -> bool {
    // TODO: Use io::ErrorKind::StorageFull when stabilized
    err.raw_os_error() == Some(28_i32)
}
