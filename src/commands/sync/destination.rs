use clap::Args as ClapArgs;
use inquire::{validator::Validation, Text};
use modman::{utils::success, Error, Profile};
use url::Url;

#[derive(ClapArgs)]
pub struct Args {
    url: Option<String>,
}

#[tokio::main]
pub async fn execute(profile: Profile, args: Args) -> Result<(), Error> {
    // ? is it a valid git remote url?
    let url = args.url.unwrap_or_else(|| {
        Text::new("Enter the URL of the repository you want to sync to")
            .with_validator(|u: &str| {
                if Url::parse(u).is_ok() {
                    Ok(Validation::Valid)
                } else {
                    Ok(Validation::Invalid(
                        "The URL you entered is invalid!".into(),
                    ))
                }
            })
            .prompt()
            .unwrap()
    });

    // we know the repo is defined thanks to the prelude so this is fine
    profile.repo.unwrap().remote("origin", &url)?;

    success(&format!(
        "Successfully set sync destination for profile {}!",
        profile.config.name
    ));

    Ok(())
}
