use anyhow::{anyhow, Result};
use log::debug;
use md5::{Digest, Md5};
use regex::Regex;

use std::path::{Path, PathBuf};

use crate::fs::get_file_name;
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
