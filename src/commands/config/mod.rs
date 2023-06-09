use clap::{Parser, Subcommand};
use modman::Error;

mod list;
// mod remove;
// mod reset;
mod set;

#[derive(Parser)]
pub struct Command {
    #[clap(subcommand)]
    pub subcommand: Subcommands,
}

#[derive(Subcommand)]
pub enum Subcommands {
    /// Set a config value
    Set(set::Args),

    /// List config values
    List(list::Args),
    // todo: Remove

    // todo: Reset
}

pub fn parse(command: Command) -> Result<(), Error> {
    let subcommand = command.subcommand;

    match subcommand {
        Subcommands::Set(args) => set::execute(args)?,
        Subcommands::List(args) => list::execute(args)?,
    }

    Ok(())
}
