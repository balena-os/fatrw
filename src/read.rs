use anyhow::{Context, Result};
use glob::glob;
use log::debug;

use std::fs::{read, remove_file};
use std::path::Path;

use crate::fs::{
    as_absolute, commit_md5sum_file, file_name_display, get_file_name, get_parent_as_string,
};

pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    debug!("Read {}", path.as_ref().display());

    let abs_path = as_absolute(path.as_ref())?;
    debug!("Absolute {}", abs_path.display());

    let content = if let Some(content) = process_md5sums(&abs_path) {
        content
    } else {
        read(&abs_path).context(format!("Failed to read target file {}", abs_path.display()))?
    };

    Ok(content)
}

fn process_md5sums(path: &Path) -> Option<Vec<u8>> {
    let file_name = get_file_name(path).ok()?;
    let parent = get_parent_as_string(path).ok()?;
    debug!("Parent directory {}", parent);

    let pattern = format!("{parent}/.{file_name}.*.*.md5sum");
    debug!("Glob pattern {}", pattern);

    let mut content = None;

    for entry in glob(&pattern)
        .context("Failed to read md5sum glob pattern")
        .ok()?
    {
        match entry {
            Ok(md5sum_path) => {
                if content.is_none() {
                    debug!("Found .md5sum file {}", file_name_display(&md5sum_path));
                    if let Ok(md5sum_content) = commit_md5sum_file(&md5sum_path, path) {
                        debug!("Md5sum file committed");
                        content = Some(md5sum_content);
                    }
                }

                let temp_path = md5sum_path.with_extension("tmp");

                remove_file(&md5sum_path).ok();
                remove_file(&temp_path).ok();
            }
            Err(e) => debug!("{:?}", e),
        }
    }

    content
}
