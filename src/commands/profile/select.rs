use clap::Args as ClapArgs;
use inquire::Select;
use modman::{
    utils::{create_slug, success},
    Error, Profile,
};

#[derive(ClapArgs)]
pub struct Args {
    name: Option<String>,
}

#[tokio::main]
pub async fn execute(args: Args) -> Result<(), Error> {
    let profile = match args.name {
        Some(name) => Profile::load(&create_slug(&name))?,
        None => {
            let profiles = Profile::load_all()?;

            Select::new("Which profile would you like to select", profiles).prompt()?
        }
    };

    profile.select()?;

    success(&format!("Selected profile {}!", profile.config.name));

    Ok(())
}
