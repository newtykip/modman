use clap::{Parser, Subcommand};
use modman::Error;

pub mod create;

#[derive(Parser)]
pub struct Command {
    #[clap(subcommand)]
    pub subcommand: Subcommands,
}

#[derive(Subcommand)]
pub enum Subcommands {
    /// Create a new profile
    Create,
}

pub fn parse(command: Command) -> Result<(), Error> {
    let subcommand = command.subcommand;

    Ok(match subcommand {
        Subcommands::Create => create::execute()?,
    })
}
