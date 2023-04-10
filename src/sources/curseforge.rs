use crate::{
    enums::{DependencyType, Loader, Sources, DependencyId},
    sources::SearchResult,
    Error, GameVersions, Mod,
};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use reqwest::Client;
use serde_json::Value;

pub const API_BASE: &str = "https://api.curseforge.com/v1";
pub const API_KEY: &str = "$2a$10$MpspbkRWQ8D5FLySK2mb/.OK/fwKQ8p7wc4aGtRBKmee8RU30wGYu";

async fn make_request(client: &Client, endpoint: String) -> Result<Value, Error> {
    Ok(client
        .get(format!("{API_BASE}/{endpoint}"))
        .header("Accept", "application/json")
        .header("x-api-key", API_KEY)
        .send()
        .await?
        .json::<Value>()
        .await?)
}

impl<'t> Mod<'t> {
    /// Search for mods on Curseforge
    pub async fn search_curseforge(
        client: &Client,
        query: &str,
        loader: Loader,
        game_versions: GameVersions<'t>,
    ) -> Result<Vec<SearchResult>, Error> {
        let search_results = make_request(
            &client,
            format!("mods/search?gameId=432&sortField=2&sortOrder=desc&pageSize=10&searchFilter=\"{query}\""),
        )
        .await?;

        let search_results = search_results["data"].as_array();

        // ensure there are search results
        if let None = search_results {
            return Ok(vec![]);
        }

        let search_results = search_results.unwrap();

        // filter mods to those which have a version which supports both the given loader and game version
        let filtered = search_results.iter().filter(|result| {
            let indexes = result["latestFilesIndexes"].as_array().unwrap();

            for index in indexes {
                let mod_loader = if let Some(mod_loader) = index["modLoader"].as_u64() {
                    loader.curseforge().contains(&(mod_loader as u8))
                } else {
                    false
                };
                let game_version = game_versions.contains(&index["gameVersion"].as_str().unwrap());

                if mod_loader && game_version {
                    return true;
                }
            }

            false
        });

        // fuzzy search!
        let matcher = SkimMatcherV2::default();
        let scores = filtered
            .clone()
            .map(|result| result["name"].as_str().unwrap())
            .enumerate()
            .map(|(i, name)| (i, matcher.fuzzy_match(name, query).unwrap_or(0)))
            .collect::<Vec<(usize, i64)>>();

        let results = filtered
            .enumerate()
            .filter(|(i, _)| scores[*i].1 != 0)
            .map(|(_, result)| SearchResult {
                name: result["name"].as_str().unwrap().to_string(),
                id: result["slug"].as_str().unwrap().to_string(),
            })
            .collect::<Vec<SearchResult>>();

        Ok(results)
    }

    /// Get a mod from CurseForge by slug.
    pub async fn from_curseforge(
        client: &Client,
        slug: &str,
        loader: Loader,
        game_versions: GameVersions<'t>,
    ) -> Result<Mod<'t>, Error> {
        let search_results =
            &make_request(&client, format!("mods/search?gameId=432&slug={slug}")).await?["data"];
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
                if let None = file.get("modLoader") {
                    false
                } else {
                    let version = &file["gameVersion"].as_str().unwrap();
                    let mod_loader = loader.curseforge().contains(
                        &(file["modLoader"]
                            .as_u64()
                            .expect("mod loader not specified") as u8),
                    );

                    let game_version = game_versions.contains(version);

                    mod_loader && game_version
                }
            })
            .map(|file| file["fileId"].as_u64().unwrap())
            .collect::<Vec<u64>>();

        let filtered_files = files
            .iter()
            .filter(|file| file_ids.contains(&file["id"].as_u64().unwrap()))
            .collect::<Vec<&Value>>();

        let latest = *filtered_files.last().unwrap();

        let mut dependencies = latest["dependencies"]
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
                    DependencyId::Version(dependency["modId"].as_u64().unwrap().to_string()),
                )
            })
            .collect::<Vec<(DependencyType, DependencyId)>>();

        dependencies.sort_by(|a, b| {
            a.1.parse::<u32>()
                .unwrap()
                .partial_cmp(&b.1.parse::<u32>().unwrap())
                .unwrap()
        });
        dependencies.dedup_by(|a, b| a.1.parse::<u32>().unwrap() == b.1.parse::<u32>().unwrap());

        Ok(Mod {
            name: project["name"]
                .as_str()
                .expect("project does not exist")
                .to_string(),
            filename: latest["fileName"].as_str().unwrap().to_string(),
            url: latest["downloadUrl"].as_str().unwrap().to_string(),
            dependencies,
            source: Sources::CurseForge,
            loader,
            game_versions
        })
    }
}
