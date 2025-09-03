mod addon;
mod config;
mod log;
mod setup;
mod wow;

use std::process::exit;

use anyhow::Result;
use clap::{Parser, Subcommand};
use config::Config;
use log::*;

use crate::wow::Instance;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Setup your WoW directory
    Setup,
}

fn main() {
    let cli = Args::parse();
    let _ = run(cli).map_err(|err| {
        log::elog(err);
        exit(1);
    });
}

fn run(cli: Args) -> Result<()> {
    let mut cfg: Config = config::load()?;
    match &cli.command {
        Some(Commands::Setup) => {
            setup::run(&mut cfg)?;
        }
        None => {}
    }

    let mut cfg: Config = config::load()?;
    if !cfg.is_initialized() {
        ilog("You don't have any WoW installation linked yet, will run setup.");
        setup::run(&mut cfg)?;
    }

    let mut instance = Instance::from_dir_path(&cfg.wow_dir_path)?;
    instance.reload_addons()?;

    Ok(())
}
