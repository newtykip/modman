use git2::Repository;
use modman::{
    utils::{alert, error, success},
    Error, Profile,
};

#[tokio::main]
pub async fn execute() -> Result<(), Error> {
    let profile = Profile::load_selected()?;

    if profile.repo.is_some() {
        alert(&format!(
            "The profile \"{}\" already has an initialized repository!",
            profile
        ));

        return Ok(());
    }

    match Repository::init(profile.path.clone()) {
        Ok(_) => success(&format!(
            "Successfully initialized repository for profile \"{}\"",
            profile
        )),
        Err(_) => error(&format!(
            "Failed to initialize repository for profile \"{}\". Have you got git installed?",
            profile
        )),
    }

    Ok(())
}
