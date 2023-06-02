use clap::{Parser, Subcommand};
use git2::Repository;
use modman::{utils::error, Error, Profile};

mod destination;
mod save;

#[derive(Parser)]
pub struct Command {
    #[clap(subcommand)]
    pub subcommand: Subcommands,
}

#[derive(Subcommand)]
pub enum Subcommands {
    /// Set the destination for syncing your selected profile
    #[clap(aliases = &["d", "dest", "remote"])]
    Destination(destination::Args),

    /// Save the state of the current profile
    #[clap(aliases = &["s"])]
    Save,
}

fn provide_profile() -> Result<Option<Profile>, Error> {
    let profile = Profile::load_selected()?;

    // if the profile doesn't have a repo, initialize one
    if profile.repo.is_none() {
        match Repository::init(profile.path.clone()) {
            Ok(_) => {}
            Err(_) => {
                error(&format!(
                    "Failed to initialize repository for profile \"{}\". Have you got git installed?",
                    profile
                ));

                return Ok(None);
            }
        }
    }

    Ok(Some(profile))
}

pub fn parse(command: Command) -> Result<(), Error> {
    let subcommand = command.subcommand;

    if let Some(profile) = provide_profile()? {
        match subcommand {
            Subcommands::Destination(args) => destination::execute(profile, args)?,
            Subcommands::Save => save::execute(profile)?,
        }
    }

    Ok(())
}
