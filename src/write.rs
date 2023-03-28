use anyhow::{Context, Result};

use log::{debug, warn};

use std::path::Path;

use crate::checksum::{generate_md5sum_path, md5sum};
use crate::fs::{
    as_absolute, commit_md5sum_file, create_file, fsync_parent_dir, is_storage_full_error,
};

pub fn write_file<P: AsRef<Path>>(path: P, content: &[u8], mode: Option<u32>) -> Result<()> {
    debug!("Write {}", path.as_ref().display());

    if let Some(m) = mode {
        debug!("Mode {:o}", m);
    }

    let abs_path = as_absolute(path.as_ref())?;
    debug!("Absolute {}", abs_path.display());

    let checksum = md5sum(content);
    debug!("Content MD5 checksum {}", checksum);

    let md5sum_path = generate_md5sum_path(&abs_path, &checksum)?;

    if let Err(err) = create_file(&md5sum_path, mode, content) {
        if is_storage_full_error(&err) {
            warn!(
                "Using unsafe write due to low space for {}",
                abs_path.display()
            );

            create_file(&abs_path, mode, content)
                .context(format!("Unsafe write filed for {}", abs_path.display()))?;

            fsync_parent_dir(&abs_path)?;

            return Ok(());
        }

        return Err(err).context(format!(
            "Failed to create checksum file {}",
            md5sum_path.display()
        ));
    }

    fsync_parent_dir(&abs_path)?;

    commit_md5sum_file(&md5sum_path, &abs_path)?;

    Ok(())
}
