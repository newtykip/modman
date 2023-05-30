use inquire::Select;
use modman::{utils::success, Error, Profile};

#[tokio::main]
pub async fn execute() -> Result<(), Error> {
    let profiles = Profile::load_all()?;

    let selected = Select::new("Which profile would you like to select", profiles).prompt()?;

    selected.select()?;

    success(&format!("Selected profile {}!", selected.config.name));

    Ok(())
}