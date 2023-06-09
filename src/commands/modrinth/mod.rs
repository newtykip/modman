use clap::{Parser, Subcommand};
use modman::Error;

mod add;
mod export;
// mod init;

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
    #[clap(alias = "e")]
    Export,

    /// Initialise a modrinth project to associate with your currently selected profile,
    #[clap(alias = "i")]
    Init,
    // todo: Publish your currently selected profile to modrinth
}

pub fn parse(command: Command) -> Result<(), Error> {
    let subcommand = command.subcommand;

    match subcommand {
        Subcommands::Add(args) => add::execute(args)?,
        Subcommands::Export => export::execute()?,
        Subcommands::Init => todo!(),
    }

    Ok(())
}
