use anyhow::{anyhow, Context, Result};
use log::{debug, warn};
use md5::{Digest, Md5};
use regex::Regex;

use std::path::{Path, PathBuf};

use crate::fs::{is_storage_full_error, read, safe_copy, safe_remove_file, safe_rename};
use crate::path::{file_name_display, get_file_name};
use crate::random::generate_random_string;

pub fn md5sum(content: &[u8]) -> String {
    format!("{:x}", Md5::digest(content))
}

pub fn extract_checksum_from_path(path: &Path) -> Result<String> {
    let filename_re =
        Regex::new(r"^\..*\.(?P<hash>[0-9a-f]{32}).md5sum$").expect("Error in md5sum file Regex");

    let file_name = get_file_name(path)?;

    filename_re
        .captures(&file_name)
        .and_then(|cap| cap.name("hash").map(|hash| hash.as_str().to_owned()))
        .ok_or_else(|| anyhow!("Cannot capture MD5 checksum from file name"))
}

pub fn generate_md5sum_path(path: &Path, checksum: &str) -> Result<PathBuf> {
    let random_suffix = generate_random_string();

    let target_name = get_file_name(path)?;

    let md5sum_name = format!(".{target_name}.{random_suffix}.{checksum}.md5sum");
    debug!("Checksum file {}", md5sum_name);

    let md5sum_path = path.with_file_name(md5sum_name);

    Ok(md5sum_path)
}

pub fn commit_md5sum_file(
    md5sum_path: &Path,
    path: &Path,
    unsafe_fallback: bool,
) -> Result<Vec<u8>> {
    debug!("Committing checksum file");

    let content = verify_checksum(md5sum_path)?;

    let temp_path = md5sum_path.with_extension("tmp");

    if let Err(err) = safe_copy(md5sum_path, &temp_path) {
        if unsafe_fallback && is_storage_full_error(&err) {
            warn!(
                "Using unsafe rename due to low space for {}",
                path.display()
            );

            safe_rename(md5sum_path, path).context(format!(
                "Failed to rename md5sum file to target file {} -> {}",
                md5sum_path.display(),
                path.display()
            ))?;

            return Ok(content);
        }

        return Err(err).context(format!(
            "Failed to copy to a temporary location {} -> {}",
            md5sum_path.display(),
            temp_path.display()
        ))?;
    }

    debug!(
        "Copied {} {}",
        file_name_display(md5sum_path),
        file_name_display(&temp_path)
    );

    safe_rename(&temp_path, path).context(format!(
        "Failed to rename temporary file to target file {} -> {}",
        temp_path.display(),
        path.display()
    ))?;
    debug!(
        "Renamed {} {}",
        file_name_display(&temp_path),
        file_name_display(path)
    );

    safe_remove_file(md5sum_path).context(format!("Failed to remove {}", md5sum_path.display()))?;
    debug!("Removed {}", file_name_display(md5sum_path));

    Ok(content)
}

pub fn verify_checksum(path: &Path) -> Result<Vec<u8>> {
    let content = read(path).context(format!("Failed to read checksum file {}", path.display()))?;

    let content_checksum = md5sum(&content);
    let file_name_checksum = extract_checksum_from_path(path)?;

    if content_checksum == file_name_checksum {
        debug!("Checksum verified");
        Ok(content)
    } else {
        Err(anyhow!(
            "Content and file name checksums do not match {} != {}",
            content_checksum,
            file_name_checksum
        ))
    }
}
