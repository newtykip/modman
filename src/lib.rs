use async_trait::async_trait;
use ferinth::{structures::version::DependencyType as FerinthDependency, Ferinth};
use reqwest::Client;
use serde_json::Value;

mod curseforge {
    pub const API_BASE: &str = "https://api.curseforge.com/v1";
    pub const API_KEY: &str = "$2a$10$MpspbkRWQ8D5FLySK2mb/.OK/fwKQ8p7wc4aGtRBKmee8RU30wGYu";
}

#[derive(Debug)]
pub enum DependencyType {
    Optional,
    Required,
    Incompatible,
    Embedded,
}

/// All of the supported mod loaders.
pub enum Loader {
    Forge,
    Fabric,
    Quilt,
}

impl Loader {
    fn as_str(&self) -> &str {
        match self {
            Loader::Forge => "forge",
            Loader::Fabric => "fabric",
            Loader::Quilt => "quilt",
        }
    }

    fn curseforge(&self) -> Vec<u8> {
        match self {
            Loader::Forge => vec![1],
            Loader::Fabric => vec![4],
            Loader::Quilt => vec![4, 5],
        }
    }
}

/// Represents a Minecraft mod.
#[derive(Debug)]
pub struct Mod {
    /// The name of the mod
    name: String,

    /// The mod's filename
    filename: String,

    /// A URL to download the mod
    url: String,

    /// The SHA1 hash of the mod's file
    sha1: String,

    dependencies: Vec<(DependencyType, String)>,
}

#[async_trait]
pub trait Find {
    /// Get a mod from Modrinth by ID.
    async fn from_modrinth(
        client: &Ferinth,
        id: &str,
        loader: Loader,
        game_versions: Option<&[&str]>,
        featured: Option<bool>,
    ) -> Result<Mod, Box<dyn std::error::Error>>;

    /// Get a mod from Curseforge by slug.
    async fn from_curseforge(
        client: &Client,
        slug: &str,
        loader: Loader,
        game_versions: Option<&[&str]>,
    ) -> Result<Mod, Box<dyn std::error::Error>>;
}

#[async_trait]
impl Find for Mod {
    async fn from_modrinth(
        client: &Ferinth,
        id: &str,
        loader: Loader,
        game_versions: Option<&[&str]>,
        featured: Option<bool>,
    ) -> Result<Mod, Box<dyn std::error::Error>> {
        let project = client.get_project(id).await?;
        let versions = client
            .list_versions_filtered(id, Some(&[loader.as_str()]), game_versions, featured)
            .await?;
        let latest = &versions[0];
        let download = &latest.files[0];

        Ok(Mod {
            name: project.title,
            filename: download.filename.clone(),
            url: download.url.to_string(),
            sha1: download.hashes.sha1.clone(),
            dependencies: latest
                .dependencies
                .iter()
                .map(|dependency| {
                    (
                        match dependency.dependency_type {
                            FerinthDependency::Embedded => DependencyType::Embedded,
                            FerinthDependency::Incompatible => DependencyType::Incompatible,
                            FerinthDependency::Optional => DependencyType::Optional,
                            _ => DependencyType::Required,
                        },
                        dependency.version_id.clone().unwrap(),
                    )
                })
                .collect::<Vec<(DependencyType, String)>>(),
        })
    }

    async fn from_curseforge(
        client: &Client,
        slug: &str,
        loader: Loader,
        game_versions: Option<&[&str]>,
    ) -> Result<Mod, Box<dyn std::error::Error>> {
        let search_results = client
            .get(format!(
                "{}/mods/search?gameId=432&slug={slug}",
                curseforge::API_BASE
            ))
            .header("x-api-key", curseforge::API_KEY)
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
            sha1: latest["hashes"].as_array().unwrap()[0]["value"]
                .as_str()
                .unwrap()
                .to_string(),
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
        })
    }
}
