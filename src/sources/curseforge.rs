use crate::{
    enums::{DependencyType, Loader, Sources},
    Mod,
};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;

pub const API_BASE: &str = "https://api.curseforge.com/v1";
pub const API_KEY: &str = "$2a$10$MpspbkRWQ8D5FLySK2mb/.OK/fwKQ8p7wc4aGtRBKmee8RU30wGYu";

#[async_trait]
pub trait FromCurse {
    /// Get a mod from Curseforge by slug.
    async fn from_curseforge(
        client: &Client,
        slug: &str,
        loader: Loader,
        game_versions: Option<&[&str]>,
    ) -> Result<Mod, Box<dyn std::error::Error>>;
}

#[async_trait]
impl FromCurse for Mod {
    async fn from_curseforge(
        client: &Client,
        slug: &str,
        loader: Loader,
        game_versions: Option<&[&str]>,
    ) -> Result<Mod, Box<dyn std::error::Error>> {
        let search_results = client
            .get(format!("{}/mods/search?gameId=432&slug={slug}", API_BASE))
            .header("x-api-key", API_KEY)
            .send()
            .await?
            .text()
            .await?;

        let search_results = &serde_json::from_str::<Value>(search_results.as_str())?["data"];
        let project = &search_results.as_array().expect("no results found")[0];
        let files = project["latestFiles"]
            .as_array()
            .expect("project has no files");
        let indexes = project["latestFilesIndexes"]
            .as_array()
            .expect("project has no files");

        let file_ids = indexes
            .iter()
            .filter(|file| {
                let mod_loader = loader.curseforge().contains(
                    &(file["modLoader"]
                        .as_u64()
                        .expect("mod loader not specified") as u8),
                );
                let game_version = if let Some(versions) = game_versions {
                    versions.contains(&file["gameVersion"].as_str().unwrap())
                } else {
                    true
                };

                mod_loader && game_version
            })
            .map(|file| file["fileId"].as_u64().unwrap())
            .collect::<Vec<u64>>();

        let filtered_files = files
            .iter()
            .filter(|file| file_ids.contains(&file["id"].as_u64().unwrap()))
            .collect::<Vec<&Value>>();

        let latest = *filtered_files.last().unwrap();

        Ok(Mod {
            name: project["name"]
                .as_str()
                .expect("project does not exist")
                .to_string(),
            filename: latest["fileName"].as_str().unwrap().to_string(),
            url: latest["downloadUrl"].as_str().unwrap().to_string(),
            dependencies: latest["dependencies"]
                .as_array()
                .unwrap()
                .iter()
                .map(|dependency| {
                    (
                        match dependency["relationType"].as_u64().unwrap() {
                            1 => DependencyType::Embedded,
                            2 => DependencyType::Optional,
                            6 => DependencyType::Incompatible,
                            _ => DependencyType::Required,
                        },
                        dependency["modId"].as_u64().unwrap().to_string(),
                    )
                })
                .collect::<Vec<(DependencyType, String)>>(),
            source: Sources::CurseForge,
        })
    }
}
