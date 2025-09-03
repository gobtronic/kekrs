use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

const APP_NAME: &str = "kekrs";
const CONFIG_NAME: &str = "config";

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub wow_dir_path: PathBuf,
}

pub fn load() -> Result<Config> {
    confy::load(APP_NAME, CONFIG_NAME).map_err(|err| err.into())
}

pub fn store(cfg: Config) -> Result<()> {
    confy::store(APP_NAME, CONFIG_NAME, cfg).map_err(|err| err.into())
}

impl Config {
    pub fn is_initialized(&self) -> bool {
        self.wow_dir_path.as_os_str() != ""
    }
}
