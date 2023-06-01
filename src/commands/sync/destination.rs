use clap::Args as ClapArgs;
use git2::{Direction, Repository};
use inquire::{validator::Validation, Text};
use modman::{
    utils::{error, success},
    Error, Profile,
};
use url::Url;

#[derive(ClapArgs)]
pub struct Args {
    url: Option<String>,
}

fn is_valid_remote(repo: Repository, url: &str) -> Result<bool, Error> {
    let mut remote = repo.remote_anonymous(url)?;
    let connection = remote.connect_auth(Direction::Fetch, None, None);

    if connection.is_err() {
        Ok(false)
    } else {
        Ok(!connection?.list()?.is_empty())
    }
}

fn validate_url(url: &str) -> bool {
    Url::parse(url).is_ok()
}

#[tokio::main]
pub async fn execute(profile: Profile, args: Args) -> Result<(), Error> {
    // we know that the profile has a repo because we checked in the prelude
    let repo = profile.repo.unwrap();
    let profile_path = profile.path.clone();

    // ensure that the url is valid
    let url = args.url.clone().unwrap_or_else(|| {
        Text::new("Enter the URL of the repository you want to sync to")
            .with_validator(move |url: &str| {
                let repo = Repository::open(profile.path.clone())?;

                if validate_url(url) && is_valid_remote(repo, url).unwrap() {
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

    if args.url.is_some()
        && (!validate_url(&url) || !is_valid_remote(Repository::open(profile_path)?, &url)?)
    {
        error("The URL you entered is invalid!");
        return Ok(());
    }

    // check if there is already a remote on the repo
    let remote_exists = !repo.remotes()?.is_empty();

    if remote_exists {
        repo.remote_set_url("origin", &url)?;
    } else {
        repo.remote("origin", &url)?;
    }

    success(&format!(
        "Successfully set sync destination for profile {}!",
        profile.config.name
    ));

    Ok(())
}
