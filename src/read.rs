use anyhow::{Context, Result};
use glob::glob;

use std::fs::{read_to_string, remove_file};
use std::path::Path;

use crate::fs::{as_absolute, commit_md5sum_file, get_file_name, get_parent_as_string};
use crate::opts::ReadArgs;

pub fn execute_read(read_args: ReadArgs) -> Result<()> {
    println!("Path: {:?}", read_args.path);

    let path = as_absolute(&read_args.path)?;
    println!("Absolute path: {:?}", path);

    let content = if let Some(content) = process_md5sums(&path) {
        content
    } else {
        read_to_string(&path).context(format!("Failed to read target file {:?}", path))?
    };

    println!("Content: {}", content);

    Ok(())
}

fn process_md5sums<P: AsRef<Path>>(path: P) -> Option<String> {
    let file_name = get_file_name(&path).ok()?;
    let parent = get_parent_as_string(&path).ok()?;
    println!("Parent: {}", parent);

    let pattern = format!("{}/.{}.*.*.md5sum", parent, file_name);
    println!("Pattern: {}", pattern);

    let mut content = None;

    for entry in glob(&pattern)
        .context("Failed to read md5sum glob pattern")
        .ok()?
    {
        match entry {
            Ok(md5sum_path) => {
                if content == None {
                    println!("Found .md5sum file: {}", md5sum_path.display());
                    if let Ok(md5sum_content) = commit_md5sum_file(&md5sum_path, &path) {
                        content = Some(md5sum_content);
                    }
                }

                let temp_path = md5sum_path.with_extension("tmp");

                remove_file(&md5sum_path).ok();
                remove_file(&temp_path).ok();
            }
            Err(e) => println!("{:?}", e),
        }
    }

    content
}
