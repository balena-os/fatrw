use anyhow::Result;

use log::debug;

use std::path::Path;

use crate::fs::get_file_mode;
use crate::read::read_file;
use crate::write::write_file;

pub fn copy_file<P: AsRef<Path>, Q: AsRef<Path>>(source: P, dest: Q) -> Result<()> {
    debug!(
        "Copy: {} {}",
        source.as_ref().display(),
        dest.as_ref().display()
    );

    let mode = get_file_mode(source.as_ref()).ok();

    write_file(dest, &read_file(source)?, mode)
}
