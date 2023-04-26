use anyhow::{anyhow, Context, Result};

use path_absolutize::Absolutize;

use std::path::Path;

use alloc::borrow::Cow;

pub fn as_absolute(path: &Path) -> Result<Cow<'_, Path>> {
    path.absolutize()
        .context(format!("Failed to absolutize {}", path.display()))
}

pub fn get_file_name(path: &Path) -> Result<String> {
    let file_name_os = path
        .file_name()
        .ok_or_else(|| anyhow!("No file name in path {}", path.display()))?;

    let file_name = file_name_os
        .to_str()
        .ok_or_else(|| anyhow!("File name is not a valid UTF-8 string {:?}", file_name_os))?;

    Ok(file_name.to_owned())
}

pub fn get_parent_as_string(path: &Path) -> Result<String> {
    let parent_path = path
        .parent()
        .ok_or_else(|| anyhow!("No parent in path {}", path.display()))?;

    let parent = parent_path.to_str().ok_or_else(|| {
        anyhow!(
            "Parent is not a valid UTF-8 string {}",
            parent_path.display()
        )
    })?;

    Ok(parent.to_owned())
}

pub fn file_name_display(path: &Path) -> Cow<'_, str> {
    path.file_name().unwrap_or_default().to_string_lossy()
}
