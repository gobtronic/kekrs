use crate::log::*;
use std::path::{Path, PathBuf};

pub fn is_valid_wow_dir_path(dir_path: &PathBuf, verbose: bool) -> bool {
    let path = Path::new(&dir_path);
    if !path.exists() {
        if verbose {
            elog("Invalid path");
        }
        return false;
    }
    if path.is_relative() {
        if verbose {
            elog("Relative path, an absolute path is expected");
        }
        return false;
    }
    if path.is_file() {
        if verbose {
            elog("Path is not a directory");
        }
    }

    let mut exe_pathbuf = dir_path.clone();
    exe_pathbuf.push("WoW.exe");
    let exe_path = Path::new(&exe_pathbuf);
    if !exe_path.exists() {
        if verbose {
            elog("No WoW.exe found, please check that the provided path is a valid WoW directory");
        }
        return false;
    }

    true
}
