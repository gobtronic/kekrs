use anyhow::{Result, bail};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct Addon {
    pub dir_path: PathBuf,
    pub toc_path: PathBuf,
}

#[derive(thiserror::Error, Debug)]
enum ValidationError {
    #[error("invalid path")]
    Invalid,
    #[error("relative path (expected absolute)")]
    Relative,
    #[error("path is not a directory")]
    NotADirectory,
    #[error("couldn't find .toc file")]
    NotAnAddonDirectory,
}

impl Addon {
    pub fn from_dir_path(dir_path: PathBuf) -> Result<Addon> {
        let toc_path = Addon::toc_path(&dir_path)?;
        Ok(Addon { dir_path, toc_path })
    }

    fn toc_path(dir_path: &PathBuf) -> Result<PathBuf> {
        let path = Path::new(dir_path);
        if !path.exists() {
            bail!(ValidationError::Invalid);
        }
        if path.is_relative() {
            bail!(ValidationError::Relative);
        }
        if path.is_file() {
            bail!(ValidationError::NotADirectory);
        }

        let mut tocs = fs::read_dir(path)?
            .filter_map(|e| e.ok())
            .filter(|e| match e.path().extension() {
                Some(ex) => ex == "toc",
                None => false,
            })
            .map(|e| e.path());
        match tocs.next() {
            Some(toc) => Ok(toc),
            None => bail!(ValidationError::NotAnAddonDirectory),
        }
    }
}
