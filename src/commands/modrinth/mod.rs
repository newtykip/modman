use clap::{Parser, Subcommand};
use modman::Error;

mod add;

#[derive(Parser)]
pub struct Command {
    #[clap(subcommand)]
    pub subcommand: Subcommands,
}

#[derive(Subcommand)]
pub enum Subcommands {
    /// Add a new mod from modrinth
    Add(add::Args),
}

pub fn parse(command: Command) -> Result<(), Error> {
    let subcommand = command.subcommand;

    match subcommand {
        Subcommands::Add(args) => add::execute(args)?,
        // todo: export
    }

    Ok(())
}
