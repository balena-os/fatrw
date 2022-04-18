use anyhow::{Context, Result};

use log::debug;

use std::path::Path;

use crate::checksum::{generate_md5sum_path, md5sum};
use crate::fs::{as_absolute, commit_md5sum_file, create_file, fsync_parent_dir, mode_from_string};

pub fn write_file<P: AsRef<Path>>(path: P, content: &str, mode: Option<&str>) -> Result<()> {
    debug!("Write: {} {:?}", path.as_ref().display(), mode);

    let mode = mode_from_string(mode)?;

    let abs_path = as_absolute(path)?;
    debug!("Absolute: {}", abs_path.display());

    let checksum = md5sum(content);
    debug!("Content MD5 checksum: {}", checksum);

    let md5sum_path = generate_md5sum_path(&abs_path, &checksum)?;

    create_file(&md5sum_path, mode, content).context(format!(
        "Failed to create checksum file {}",
        md5sum_path.display()
    ))?;

    fsync_parent_dir(&abs_path)?;

    commit_md5sum_file(md5sum_path, &abs_path)?;

    Ok(())
}
