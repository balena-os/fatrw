use anyhow::{Context, Result};

use crate::checksum::{generate_md5sum_path, md5sum};
use crate::fs::{as_absolute, commit_md5sum_file, create_file, fsync_parent_dir, mode_from_string};
use crate::opts::WriteArgs;

pub fn execute_write(write_args: WriteArgs) -> Result<()> {
    println!("Path: {:?}", write_args.path);

    let mode = mode_from_string(write_args.mode.as_deref())?;

    let path = as_absolute(&write_args.path)?;
    println!("Absolute path: {:?}", path);

    let checksum = md5sum(&write_args.content);
    println!("MD5 checksum: {}", checksum);

    let md5sum_path = generate_md5sum_path(&path, &checksum)?;

    create_file(&md5sum_path, mode, &write_args.content)
        .context(format!("Failed to create checksum file {:?}", md5sum_path))?;
    println!("Created {:?}", md5sum_path.file_name().unwrap());

    fsync_parent_dir(&path)?;

    commit_md5sum_file(md5sum_path, &path)?;

    Ok(())
}
