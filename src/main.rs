mod commands;
use clap::Parser;
use commands::{Commands, Value};
use modman::Error;

// todo: profile system with git repos hidden somewhere which can be synchronised on command

fn main() -> Result<(), Error> {
    let command = Value::parse().command;

    Ok(match command {
        Commands::Init(options) => commands::init::execute(options)?,
    })
}
