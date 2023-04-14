use clap::{Parser, Subcommand};
use modman::Error;

pub mod prism;

#[derive(Parser)]
pub struct Command {
    #[clap(subcommand)]
    pub subcommand: Subcommands,
}

#[derive(Subcommand)]
pub enum Subcommands {
    /// Export the selected profile to a Prism instance
    Prism,
}

pub fn parse(command: Command) -> Result<(), Error> {
    let subcommand = command.subcommand;

    Ok(match subcommand {
        Subcommands::Prism => prism::execute()?,
    })
}
