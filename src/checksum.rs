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

pub fn extract_checksum_from_path<P: AsRef<Path>>(path: P) -> Result<String> {
    let filename_re = Regex::new(r"^\..*\.(?P<hash>[0-9a-f]{32}).md5sum$").unwrap();

    let file_name = get_file_name(path)?;

    filename_re
        .captures(&file_name)
        .and_then(|cap| cap.name("hash").map(|hash| hash.as_str().to_string()))
        .ok_or_else(|| anyhow!("Cannot capture MD5 checksum from file name"))
}

pub fn generate_md5sum_path<P: AsRef<Path>>(path: P, checksum: &str) -> Result<PathBuf> {
    let path = path.as_ref();

    let random_suffix = generate_random_string();

    let target_name = get_file_name(&path)?;

    let md5sum_name = format!(".{}.{}.{}.md5sum", target_name, random_suffix, checksum);
    debug!("Checksum file name: {}", md5sum_name);

    let md5sum_path = path.with_file_name(md5sum_name);

    Ok(md5sum_path)
}
