#[macro_use]
extern crate prettytable;

use clap::Parser;
use human_panic::setup_panic;
use modman::{utils::MODMAN_DIR, Config, Error};
use std::fs;

mod commands;
use commands::*;

fn main() -> Result<(), Error> {
    setup_panic!();

    // ensure the modman directory exists
    if !MODMAN_DIR.exists() {
        fs::create_dir_all(MODMAN_DIR.clone())?;
        fs::File::create(MODMAN_DIR.join(".selected"))?;
        Config::default().save()?;
    }

    let command = Value::parse().command;

    match command {
        Commands::Profile(subcommand) => profile::parse(subcommand)?,
        Commands::Modrinth(subcommand) => modrinth::parse(subcommand)?,
        Commands::Config(subcommand) => config::parse(subcommand)?,
    }

    Ok(())
}
