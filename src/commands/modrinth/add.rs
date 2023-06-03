use clap::Args as ClapArgs;
use inquire::{Confirm, Select};
use modman::{utils::success, Error, ModrinthMod, Profile};

#[derive(ClapArgs)]
pub struct Args {
    query: String,
}

#[tokio::main]
pub async fn execute(args: Args) -> Result<(), Error> {
    let selected_profile = Profile::load_selected()?;

    let results = ModrinthMod::search(
        args.query,
        selected_profile.loader,
        vec![&selected_profile.config.versions.minecraft],
    )
    .await?;

    // make the user select a search result
    let selected_mod = if results.len() == 1 {
        results[0].clone()
    } else {
        Select::new("Please select a mod", results).prompt()?
    };

    let mcmod = ModrinthMod::from_project(
        selected_mod.id,
        selected_profile.loader,
        vec![&selected_profile.config.versions.minecraft],
    )
    .await?;

    // ask if dependencies should be installed
    let dependencies = mcmod.resolve_dependencies(true).await?;

    if !dependencies.is_empty() {
        println!("The following dependencies have been found:");
        for dependency in &dependencies {
            println!("  {}", dependency.data.name);
        }

        if Confirm::new("Would you like to add them?").prompt()? {
            for dependency in dependencies {
                selected_profile.add_mod(&dependency.data)?;
            }
        }
    }

    selected_profile.add_mod(&mcmod.data)?;

    success(&format!(
        "Successfully added {}! ({})",
        mcmod.data.name, mcmod.data.filename
    ));

    Ok(())
}
