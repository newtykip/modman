use super::SearchResult;
use crate::{
    enums::{DependencyType, Loader, Sources},
    Error, GameVersions, Mod,
};
use ferinth::{structures::version::DependencyType as FerinthDependency, Ferinth};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use reqwest::Client;
use serde_json::Value;

impl<'v> Mod {
    /// Search for mods on Modrinth.
    pub async fn search_modrinth(
        client: &Client,
        query: &str,
        loader: Loader,
        game_versions: GameVersions<'v>,
    ) -> Result<Vec<SearchResult>, Error> {
        let results = client
            .get(format!("https://api.modrinth.com/v2/search?query={query}&facets=[[\"project_type:mod\"],[\"categories:{}\"{}],[{}]]", loader.as_str(), match loader { Loader::Quilt => ",\"categories:fabric\"", _ => "" }, game_versions.iter().enumerate().map(|(i, version)| format!("\"versions:{version}\"{}", if i != game_versions.len() - 1 { "," } else { "" })).collect::<String>()).as_str())
            .send()
            .await?
            .json::<Value>()
            .await?;

        let results = results["hits"]
            .as_array()
            .unwrap()
            .iter()
            .map(|result| SearchResult {
                name: result["title"].as_str().unwrap().to_string(),
                id: result["project_id"].as_str().unwrap().to_string(),
            });

        // fuzzy search
        let matcher = SkimMatcherV2::default();
        let scores = results
            .clone()
            .map(|result| result.name)
            .enumerate()
            .map(|(i, name)| (i, matcher.fuzzy_match(name.as_str(), query).unwrap_or(0)))
            .collect::<Vec<(usize, i64)>>();

        let results = results
            .enumerate()
            .filter(|(i, _)| scores[*i].1 != 0)
            .map(|(_, x)| x)
            .collect::<Vec<SearchResult>>();

        Ok(results)
    }

    /// Get a mod from Modrinth by project ID.
    pub async fn from_modrinth(
        client: &Ferinth,
        id: &str,
        loader: Loader,
        game_versions: GameVersions<'v>,
        featured: Option<bool>,
    ) -> Result<Mod, Error> {
        let project = client.get_project(id).await?;
        let versions = client
            .list_versions_filtered(id, Some(&[loader.as_str()]), Some(game_versions), featured)
            .await?;
        let latest = &versions[0];
        let download = &latest.files[0];

        Ok(Mod {
            name: project.title,
            filename: download.filename.clone(),
            url: download.url.to_string(),
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
            source: Sources::Modrinth,
        })
    }
}
