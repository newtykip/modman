use clap::Args as ClapArgs;
use inquire::{Confirm, Select};
use modman::{
    utils::{create_slug, error, success},
    Error, Profile,
};

#[derive(ClapArgs)]
pub struct Args {
    name: Option<String>,
}

#[tokio::main]
pub async fn execute(args: Args) -> Result<(), Error> {
    let profile = match args.name {
        Some(name) => {
            let profile = Profile::load(&create_slug(&name));

            if profile.is_err() {
                error(&format!("Profile {} does not exist!", name));
                return Ok(());
            }

            profile?
        }
        None => Profile::get_selected()?,
    };

    // ask for confirmation
    let approved = Confirm::new(&format!(
        "Are you sure you want to delete the profile {}?",
        profile.config.name
    ))
    .with_default(false)
    .prompt()?;

    if !approved {
        Ok(())
    } else {
        // if the profile was selected ask for a new profile to select
        if profile.is_selected()? {
            let profiles = Profile::load_all()?;

            let new_profile = Select::new(
                "Select a profile to switch to",
                profiles
                    .iter()
                    .filter(|p| p != &&profile)
                    .collect::<Vec<_>>(),
            )
            .prompt()?;

            new_profile.select()?;
        }

        profile.delete()?;
        success(&format!("Deleted profile {}!", profile.config.name));

        Ok(())
    }
}
