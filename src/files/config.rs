use ferinth::Ferinth;
use inquire::{Select, Text};
use quickxml_to_serde::xml_str_to_json;
use reqwest::Client;
use serde::{Serialize, Deserialize};
use std::{path::PathBuf, io::Write};

use crate::{Error, Loader};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    name: String,
    author: String,
    version: String,
    versions: Versions,
}

#[derive(Serialize, Deserialize, Debug)]
struct Versions {
    minecraft: String,
    forge: Option<String>,
    fabric: Option<String>,
    quilt: Option<String>,
}

impl Config {
    pub fn read(path: PathBuf) -> Result<Config, toml::de::Error> {
        toml::from_str::<Config>(std::fs::read_to_string(path).unwrap().as_str())
    }

    pub fn write(&self, dir: PathBuf) -> Result<(), Error> {
        let content = toml::to_string(&self)?;
        std::fs::create_dir_all(&dir)?;
        let mut file = std::fs::File::create(dir.join("pack.toml"))?;

        file.write(content.as_bytes())?;

        Ok(())
    }

    pub async fn prompt(client: Option<Client>, ferinth: Option<Ferinth>) -> Result<Config, Error> {
        // prepare the versions list
        let client = client.unwrap_or(Client::new());
        let ferinth = ferinth.unwrap_or(Ferinth::default());

        let mut versions = ferinth
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

        versions.pop();

        // prompt the user for the modpack's information
        let name = Text::new("What is the name of your modpack?")
            .with_default(
                std::env::current_dir()?
                    .iter()
                    .last()
                    .unwrap()
                    .to_str()
                    .unwrap(),
            )
            .prompt()?;

        let author = Text::new("Who is the author of this modpack?")
            .with_default(std::env::var("USER").unwrap().as_str())
            .prompt()?;

        let version = Text::new("What is the version of this modpack?")
            .with_default("1.0.0")
            .prompt()?;

        let minecraft_version =
            Select::new("What Minecraft version do you want to use?", versions).prompt()?;

        // now ask about the mod loader
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

        // fetch the available versions of the mod loader
        let loader_versions = xml_str_to_json(client.get(match mod_loader {
			Loader::Forge => "https://files.minecraftforge.net/maven/net/minecraftforge/forge/maven-metadata.xml",
			Loader::Fabric => "https://maven.fabricmc.net/net/fabricmc/fabric-loader/maven-metadata.xml",
			Loader::Quilt => "https://maven.quiltmc.org/repository/release/org/quiltmc/quilt-loader/maven-metadata.xml",
		}).send().await?.text().await?.as_str(), &quickxml_to_serde::Config::new_with_defaults())?;

        // find out which version of the mod loader to use
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
                .unwrap()[1]
                .to_string(),
            _ => {
                let mut loader_versions = loader_versions
                    .filter(|v| v.split(".").collect::<Vec<&str>>().len() <= 3)
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>();

                loader_versions.reverse();

                Select::new(
                    "What version of the mod loader do you want to use?",
                    loader_versions,
                )
                .prompt()?
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

        Ok(config)
    }
}
