use anyhow::{Context, Result};
use glob::glob;
use log::debug;

use std::fs::{read_to_string, remove_file};
use std::path::Path;

use crate::fs::{as_absolute, commit_md5sum_file, get_file_name, get_parent_as_string};
use crate::opts::ReadArgs;

pub fn execute_read(read_args: ReadArgs) -> Result<()> {
    debug!("Read: {}", read_args.path.display());

    let path = as_absolute(&read_args.path)?;
    debug!("Absolute: {}", path.display());

    let content = if let Some(content) = process_md5sums(&path) {
        content
    } else {
        read_to_string(&path).context(format!("Failed to read target file {}", path.display()))?
    };

    print!("{}", content);

    Ok(())
}

fn process_md5sums<P: AsRef<Path>>(path: P) -> Option<String> {
    let file_name = get_file_name(&path).ok()?;
    let parent = get_parent_as_string(&path).ok()?;
    debug!("Parent directory: {}", parent);

    let pattern = format!("{}/.{}.*.*.md5sum", parent, file_name);
    debug!("Glob pattern: {}", pattern);

    let mut content = None;

    for entry in glob(&pattern)
        .context("Failed to read md5sum glob pattern")
        .ok()?
    {
        match entry {
            Ok(md5sum_path) => {
                if content == None {
                    debug!("Found .md5sum file: {}", md5sum_path.display());
                    if let Ok(md5sum_content) = commit_md5sum_file(&md5sum_path, &path) {
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
