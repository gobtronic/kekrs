use anyhow::Result;
use std::path::{Path, PathBuf};

#[derive(thiserror::Error, Debug)]
enum ValidationError {
    #[error("invalid path")]
    Invalid,
    #[error("relative path (expected absolute)")]
    Relative,
    #[error("path is not a directory")]
    NotADirectory,
    #[error("couldn't find WoW.exe")]
    NotAWoWDirectory,
}

pub fn validate_dir_path(dir_path: &PathBuf) -> Result<()> {
    let path = Path::new(&dir_path);
    if !path.exists() {
        return Err(ValidationError::Invalid.into());
    }
    if path.is_relative() {
        return Err(ValidationError::Relative.into());
    }
    if path.is_file() {
        return Err(ValidationError::NotADirectory.into());
    }

    let mut exe_pathbuf = dir_path.clone();
    exe_pathbuf.push("WoW.exe");
    let exe_path = Path::new(&exe_pathbuf);
    if !exe_path.exists() {
        return Err(ValidationError::NotAWoWDirectory.into());
    }

    Ok(())
}
