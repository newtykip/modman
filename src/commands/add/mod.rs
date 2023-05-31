use clap::{Parser, Subcommand};
use modman::Error;

mod modrinth;

#[derive(Parser)]
pub struct Command {
    #[clap(subcommand)]
    pub subcommand: Subcommands,
}

#[derive(Subcommand)]
pub enum Subcommands {
    /// Add a new mod from modrinth
    Modrinth(modrinth::Args),
}

pub fn parse(command: Command) -> Result<(), Error> {
    let subcommand = command.subcommand;

    match subcommand {
        Subcommands::Modrinth(args) => modrinth::execute(args)?,
    }

    Ok(())
}
