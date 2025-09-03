use std::path::PathBuf;

use anyhow::Result;
use dialoguer::{Input, theme::ColorfulTheme};

use crate::{
    config::{self, Config},
    wow,
};

pub fn run(cfg: &mut Config) -> Result<()> {
    let dir_path = prompt_dir_path();
    wow::validate_dir_path(&dir_path)?;
    cfg.wow_dir_path = dir_path;
    config::store(cfg.clone())?;
    Ok(())
}

fn prompt_dir_path() -> PathBuf {
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("WoW directory path")
        .interact_text()
        .unwrap();

    input.into()
}
