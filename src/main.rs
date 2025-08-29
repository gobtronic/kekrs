mod config;
mod log;
mod wow;

use std::{path::PathBuf, process::exit};

use anyhow::Result;
use config::Config;
use dialoguer::{Input, theme::ColorfulTheme};
use log::*;

fn main() -> Result<()> {
    let mut cfg: Config = confy::load("kekrs", "config")?;
    if cfg.wow_dir_path.as_os_str() == "" {
        ilog("You don't have any WoW installation linked yet, will run setup.");
        let wow_dir_path = prompt_wow_dir_path();
        if !wow::is_valid_wow_dir_path(&wow_dir_path, true) {
            exit(1);
        }
        cfg.wow_dir_path = wow_dir_path;
        confy::store("kekrs", "config", cfg)?;
    }

    Ok(())
}

fn prompt_wow_dir_path() -> PathBuf {
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("WoW directory path")
        .interact_text()
        .unwrap();

    input.into()
}
