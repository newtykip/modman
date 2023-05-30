use clap::{Parser, Subcommand};
use modman::{utils::error, Error, Profile};

mod destination;
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

    /// Set the destination for the sync
    #[clap(alias = "dest")]
    Destination,
}

fn prelude() -> Result<Option<Profile>, Error> {
    let profile = Profile::load_selected()?;

    if profile.repo.is_none() {
        error(&format!(
            "The profile \"{}\" does not have an initialized repository!",
            profile
        ));

        return Ok(None);
    }

    Ok(Some(profile))
}

pub fn parse(command: Command) -> Result<(), Error> {
    let subcommand = command.subcommand;

    match subcommand {
        Subcommands::Init => init::execute()?,
        Subcommands::Destination => {
            let profile = prelude()?;

            if let Some(profile) = profile {
                destination::execute(profile)?;
            }
        }
    }

    Ok(())
}
