use clap::{Parser, Subcommand};
use modman::{utils::error, Error, Profile};

mod destination;
mod init;
mod save;
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
    Destination(destination::Args),

    // Save the state of the current profile
    Save,
}

fn provide_profile(function: impl Fn(Profile) -> Result<(), Error>) -> Result<(), Error> {
    let profile = Profile::load_selected()?;

    if profile.repo.is_none() {
        error(&format!(
            "The profile \"{}\" does not have an initialized repository!",
            profile
        ));
    } else {
        function(profile)?;
    }

    Ok(())
}

pub fn parse(command: Command) -> Result<(), Error> {
    let subcommand = command.subcommand;

    match subcommand {
        Subcommands::Init => init::execute()?,
        Subcommands::Destination(args) => {
            provide_profile(move |profile| destination::execute(profile, args.to_owned()))?
        }
        Subcommands::Save => provide_profile(save::execute)?,
    }

    Ok(())
}
