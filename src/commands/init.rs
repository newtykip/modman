use clap::Args as ClapArgs;
use ferinth::Ferinth;
use inquire::{Select, Text};
use modman::config::Versions;
use modman::{Config, Error, Loader};
use quickxml_to_serde::xml_str_to_json;
use reqwest::Client;

#[derive(ClapArgs, Debug)]
pub struct Args {
    /// Autofill fields where possible
    #[arg(short, long, default_value_t = false)]
    yes: bool,
}

#[tokio::main]
pub async fn execute(options: Args) -> Result<(), Error> {
    let current_dir = std::env::current_dir()?;

    // prepare the game versions list
    let client = Client::new();
    let ferinth = Ferinth::default();

    let mut game_versions = ferinth
        .list_game_versions()
        .await?
        .iter()
        .filter(|v| {
            v.major
                && v.version.split(".").collect::<Vec<&str>>()[1]
                    .parse::<u8>()
                    .unwrap()
                    >= 7
        })
        .map(|v| v.version.clone())
        .collect::<Vec<String>>();

    game_versions.pop();

    // prompt the user for the modpack's information

    let (name, author, version) = {
        let name_default = current_dir.iter().last().unwrap().to_str().unwrap();

        let author_default = std::env::var("USER").unwrap();
        let version_default = "1.0.0";

        if !options.yes {
            // 1. name of the modpack
            (
                Text::new("What is the name of your modpack?")
                    .with_default(name_default)
                    .prompt()?,
                // 2. author of the modpack
                Text::new("Who is the author of this modpack?")
                    .with_default(author_default.as_str())
                    .prompt()?,
                // 3. version of the modpack
                Text::new("What is the version of this modpack?")
                    .with_default(version_default)
                    .prompt()?,
            )
        } else {
            (
                name_default.to_string(),
                author_default,
                version_default.to_string(),
            )
        }
    };

    // 4. the game version the modpack should run on
    let minecraft_version =
        Select::new("What Minecraft version do you want to use?", game_versions).prompt()?;

    // 5. the mod loader the modpack should run on
    let mod_loader = match Select::new(
        "What mod loader do you want to use?",
        vec!["Forge", "Fabric", "Quilt"],
    )
    .prompt()?
    {
        "Forge" => Loader::Forge,
        "Fabric" => Loader::Fabric,
        "Quilt" => Loader::Quilt,
        _ => unreachable!(),
    };

    // find the latest version of the mod laoder
    let loader_versions = xml_str_to_json(client.get(match mod_loader {
		Loader::Forge => "https://files.minecraftforge.net/maven/net/minecraftforge/forge/maven-metadata.xml",
		Loader::Fabric => "https://maven.fabricmc.net/net/fabricmc/fabric-loader/maven-metadata.xml",
		Loader::Quilt => "https://maven.quiltmc.org/repository/release/org/quiltmc/quilt-loader/maven-metadata.xml",
	}).send().await?.text().await?.as_str(), &quickxml_to_serde::Config::new_with_defaults())?;

    let loader_versions = loader_versions["metadata"]["versioning"]["versions"]["version"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap());

    let loader_version: String = match mod_loader {
        Loader::Forge => loader_versions
            .map(|v| v.split("-").collect::<Vec<&str>>())
            .filter(|v| v[0] == minecraft_version)
            .collect::<Vec<Vec<&str>>>()
            .first()
            .unwrap()
            .join("-")
            .to_string(),
        _ => {
            let mut loader_versions = loader_versions
                .filter(|v| v.split(".").collect::<Vec<&str>>().len() <= 3)
                .map(|v| v.to_string())
                .collect::<Vec<String>>();

            loader_versions.reverse();

            loader_versions.first().unwrap().to_owned()
        }
    };

    // write all of this data into the schema
    let config = Config {
        name,
        author,
        version,
        versions: Versions {
            minecraft: minecraft_version,
            forge: if mod_loader == Loader::Forge {
                Some(loader_version.clone())
            } else {
                None
            },
            fabric: if mod_loader == Loader::Fabric {
                Some(loader_version.clone())
            } else {
                None
            },
            quilt: if mod_loader == Loader::Quilt {
                Some(loader_version.clone())
            } else {
                None
            },
        },
    };

    // save the information to a file
    config.write(current_dir)?;

    Ok(())
}
