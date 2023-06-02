use clap::{Parser, Subcommand};
use modman::Error;

mod backup;
mod create;
mod select;
mod view;

#[derive(Parser)]
pub struct Command {
    #[clap(subcommand)]
    pub subcommand: Subcommands,
}

#[derive(Subcommand)]
pub enum Subcommands {
    /// Create a new profile
    #[clap(aliases = &["c", "new"])]
    Create,

    /// Select a profile
    #[clap(aliases = &["s", "choose"])]
    Select,

    /// Backup your profiles using git
    #[clap(aliases = &["b", "sync"])]
    Backup(backup::Command),

    /// View a profile
    #[clap(aliases = &["v", "current"])]
    View(view::Args),
    // todo: Modify

    // todo: Delete
}

pub fn parse(command: Command) -> Result<(), Error> {
    let subcommand = command.subcommand;

    match subcommand {
        Subcommands::Create => create::execute()?,
        Subcommands::Select => select::execute()?,
        Subcommands::View(args) => view::execute(args)?,
        Subcommands::Backup(subcommand) => backup::parse(subcommand)?,
    }

    Ok(())
}
