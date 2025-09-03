use anyhow::{Result, bail};
use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::addon::Addon;

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
        bail!(ValidationError::Invalid);
    }
    if path.is_relative() {
        bail!(ValidationError::Relative);
    }
    if path.is_file() {
        bail!(ValidationError::NotADirectory);
    }

    let mut exe_pathbuf = dir_path.clone();
    exe_pathbuf.push("WoW.exe");
    let exe_path = Path::new(&exe_pathbuf);
    if !exe_path.exists() {
        bail!(ValidationError::NotAWoWDirectory);
    }

    Ok(())
}

pub struct Instance {
    pub root_dir_path: PathBuf,
    pub addons_dir_path: PathBuf,
    pub addons: Vec<Addon>,
}

impl Instance {
    pub fn from_dir_path(dir_path: &PathBuf) -> Result<Instance> {
        validate_dir_path(dir_path)?;
        let mut addons_dir_path = dir_path.clone();
        addons_dir_path.push("Interface");
        addons_dir_path.push("AddOns");
        if !Path::new(&addons_dir_path).is_dir() {
            bail!("addons directory path is invalid");
        }
        Ok(Instance {
            root_dir_path: dir_path.clone(),
            addons_dir_path,
            addons: vec![],
        })
    }

    pub fn reload_addons(&mut self) -> Result<()> {
        let addons = fs::read_dir(self.addons_dir_path.clone())?
            .filter_map(|e| e.ok())
            .filter_map(|e| Addon::from_dir_path(e.path()).ok());
        self.addons = addons.collect();
        Ok(())
    }
}
