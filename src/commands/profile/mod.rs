use clap::{Parser, Subcommand};
use modman::Error;

mod backup;
mod create;
mod delete;
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
    Select(select::Args),

    /// Backup your profiles using git
    #[clap(aliases = &["b", "sync"])]
    Backup(backup::Command),

    /// View a profile
    #[clap(aliases = &["v", "current"])]
    View(view::Args),

    /// Delete a profilr
    #[clap(aliases = &["d", "remove"])]
    Delete(delete::Args), // todo: Modify
}

pub fn parse(command: Command) -> Result<(), Error> {
    let subcommand = command.subcommand;

    match subcommand {
        Subcommands::Create => create::execute()?,
        Subcommands::Select(args) => select::execute(args)?,
        Subcommands::View(args) => view::execute(args)?,
        Subcommands::Backup(subcommand) => backup::parse(subcommand)?,
        Subcommands::Delete(args) => delete::execute(args)?,
    }

    Ok(())
}
