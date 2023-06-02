use clap::{Parser, Subcommand};
use modman::Error;

mod add;
mod export;

#[derive(Parser)]
pub struct Command {
    #[clap(subcommand)]
    pub subcommand: Subcommands,
}

#[derive(Subcommand)]
pub enum Subcommands {
    /// Add a new mod from modrinth
    #[clap(alias = "a")]
    Add(add::Args),

    /// Export your currently selected profile to an .mrpack
    Export,
}

pub fn parse(command: Command) -> Result<(), Error> {
    let subcommand = command.subcommand;

    match subcommand {
        Subcommands::Add(args) => add::execute(args)?,
        Subcommands::Export => export::execute()?,
    }

    Ok(())
}
