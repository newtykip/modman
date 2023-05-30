use inquire::Text;
use modman::{utils::error, Error, Profile};
use url::Url;

#[tokio::main]
pub async fn execute(profile: Profile) -> Result<(), Error> {
    let mut url: String;

    // ? is it a valid git remote url?
    loop {
        url = Text::new("Enter the URL of the repository you want to sync to")
            .with_help_message("This is the URL of the repository you want to sync to")
            .prompt()?;

        match Url::parse(&url) {
            Ok(_) => break,
            Err(_) => {
                error("The URL you entered is invalid!");
            }
        }
    }

    // we know the repo is defined thanks to the prelude so this is fine
    profile.repo.unwrap().remote("origin", &url)?;

    Ok(())
}
