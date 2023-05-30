use std::fs;

use clap::Parser;
use human_panic::setup_panic;
use modman::{utils::modman_dir, Error};

mod commands;
use commands::*;

fn main() -> Result<(), Error> {
    setup_panic!();

    // ensure the modman directory exists
    let modman = modman_dir();

    if !modman.exists() {
        fs::create_dir_all(&modman).unwrap();
        fs::File::create(modman.join(".selected")).unwrap();
    }

    let command = Value::parse().command;

    match command {
        Commands::Profile(subcommand) => profile::parse(subcommand)?,
        Commands::Sync(subcommand) => sync::parse(subcommand)?,
    }

    Ok(())
}
