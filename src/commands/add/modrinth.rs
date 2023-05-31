use crate::Error;
use clap::Args as ClapArgs;
use inquire::Select;
use modman::{ModrinthMod, Profile};

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

    let mcmod = ModrinthMod::new(
        selected_mod.id,
        selected_profile.loader,
        vec![&selected_profile.config.versions.minecraft],
    )
    .await?;

    mcmod.data.write(
        selected_profile
            .path
            .join("mods")
            .join(format!("{}.mm.toml", mcmod.data.slug)),
    )?;

    Ok(())
}
