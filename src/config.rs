use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub wow_dir_path: PathBuf,
}
