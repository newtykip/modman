use clap::{Parser, Subcommand};
use modman::Error;

mod init;

#[derive(Parser)]
pub struct Command {
    #[clap(subcommand)]
    pub subcommand: Subcommands,
}

#[derive(Subcommand)]
pub enum Subcommands {
    /// Initalise a git repository for your selected profile
    Init,
}

pub fn parse(command: Command) -> Result<(), Error> {
    let subcommand = command.subcommand;

    match subcommand {
        Subcommands::Init => init::execute()?,
    }

    Ok(())
}
