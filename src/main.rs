use clap::Parser;
use human_panic::setup_panic;
use modman::Error;

mod commands;
use commands::*;

fn main() -> Result<(), Error> {
    setup_panic!();

    let command = Value::parse().command;

    Ok(match command {
        Commands::Profile(subcommand) => profile::parse(subcommand)?,
    })
}
