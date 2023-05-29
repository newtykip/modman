use ferinth::Ferinth;
use inquire::{validator::Validation, Select, Text};
use modman::{utils::modman_dir, Config, ConfigVersions, Error, Index, Loader, Toml};
use nanoid::nanoid;
use quickxml_to_serde::xml_str_to_json;
use rayon::prelude::*;
use reqwest::Client;

// todo: make sure names can not be duplicated

#[tokio::main]
pub async fn execute() -> Result<(), Error> {
    // create the directory
    let uuid = nanoid!();
    let modman_directory = modman_dir();
    let modman_existed = modman_directory.exists();
    let pack_directory = modman_directory.join(&uuid);

    if !pack_directory.exists() {
        std::fs::create_dir_all(pack_directory.clone())?;
    }

    // load the index file
    let index_file = if modman_existed {
        Index::open(modman_directory.join("index.toml"))?
    } else {
        Index::new(modman_directory.join("index.toml"))
    };

    let used_names = index_file
        .0
        .par_iter()
        .filter(|e| e.0 != "path") // remove the stored path key
        .map(|e| e.1.to_string())
        .collect::<Vec<String>>();

    // prepare the game versions list
    let client = Client::new();
    let ferinth = Ferinth::default();

    let mut game_versions = ferinth
        .list_game_versions()
        .await?
        .par_iter()
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
    let (name, author, version, minecraft_version) = (
        // 1. name of the modpack
        Text::new("What is the name of your modpack?")
            .with_validator(move |name: &str| {
                if name.len() == 0 {
                    Ok(Validation::Invalid("Name can not be empty".into()))
                } else if used_names.contains(&name.to_string()) {
                    Ok(Validation::Invalid("Name has already been used".into()))
                } else {
                    Ok(Validation::Valid)
                }
            })
            .prompt()?,
        // 2. author of the modpack
        Text::new("Who is the author of this modpack?")
            .with_default(std::env::var("USER").unwrap().as_str())
            .prompt()?,
        // 3. version of the modpack
        Text::new("What is the version of this modpack?")
            .with_default("1.0.0")
            .prompt()?,
        // 4. the game version the modpack should run on
        Select::new("What Minecraft version do you want to use?", game_versions).prompt()?,
    );

    // 5. the mod loader the modpack should run on
    let mod_loader = {
        let loader = Select::new(
            "Which mod loader do you want to use?",
            vec!["Forge", "Fabric", "Quilt"],
        )
        .prompt()?;

        match loader {
            "Forge" => Loader::Forge,
            "Fabric" => Loader::Fabric,
            "Quilt" => Loader::Quilt,
            _ => unreachable!(),
        }
    };

    // find the latest version of the mod loader
    // todo: maybe allow for selection of versions? how useful is this?
    let loader_versions = xml_str_to_json(client.get(match mod_loader {
		Loader::Forge => "https://files.minecraftforge.net/maven/net/minecraftforge/forge/maven-metadata.xml",
		Loader::Fabric => "https://maven.fabricmc.net/net/fabricmc/fabric-loader/maven-metadata.xml",
		Loader::Quilt => "https://maven.quiltmc.org/repository/release/org/quiltmc/quilt-loader/maven-metadata.xml",
	}).send().await?.text().await?.as_str(), &quickxml_to_serde::Config::new_with_defaults())?;

    let loader_versions = loader_versions["metadata"]["versioning"]["versions"]["version"]
        .as_array()
        .unwrap()
        .par_iter()
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
        name: name.clone(),
        author,
        version,
        versions: ConfigVersions {
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

    // save to profile.toml
    config.write(pack_directory.join("pack.toml"))?;

    // add to index.toml
    index_file.append(uuid, name)?;

    Ok(())
}
