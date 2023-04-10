use super::SearchResult;
use crate::{
    enums::{DependencyId, DependencyType, Loader, Sources},
    Error, GameVersions, Mod,
};
use ferinth::{
    structures::version::{DependencyType as FerinthDependency, Version},
    Ferinth,
};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use reqwest::Client;
use serde_json::Value;

fn construct_mod<'t>(
    name: &String,
    version: &Version,
    loader: Loader,
    game_versions: GameVersions<'t>,
) -> Mod<'t> {
    let download = &version.files[0];

    Mod {
        name: name.clone(),
        filename: download.filename.clone(),
        url: download.url.to_string(),
        dependencies: version
            .dependencies
            .iter()
            .filter(|dependency| dependency.version_id.is_some() || dependency.project_id.is_some())
            .map(|dependency| {
                (
                    match dependency.dependency_type {
                        FerinthDependency::Embedded => DependencyType::Embedded,
                        FerinthDependency::Incompatible => DependencyType::Incompatible,
                        FerinthDependency::Optional => DependencyType::Optional,
                        _ => DependencyType::Required,
                    },
                    if let Some(version_id) = dependency.clone().version_id {
                        DependencyId::Version(version_id)
                    } else {
                        DependencyId::Project(dependency.clone().project_id.unwrap())
                    },
                )
            })
            .collect::<Vec<(DependencyType, DependencyId)>>(),
        source: Sources::Modrinth,
        loader,
        game_versions,
    }
}

impl<'t> Mod<'t> {
    /// Search for mods on Modrinth.
    pub async fn search_modrinth(
        client: &Client,
        query: &str,
        loader: Loader,
        game_versions: GameVersions<'t>,
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
        id: &str,
        loader: Loader,
        game_versions: GameVersions<'t>,
        featured: Option<bool>,
    ) -> Result<Option<Mod<'t>>, Error> {
        let ferinth = Ferinth::default();
        let project = ferinth.get_project(id).await?;
        let versions = ferinth
            .list_versions_filtered(id, Some(&[loader.as_str()]), Some(game_versions), featured)
            .await?;

        if versions.len() == 0 {
            return Ok(None);
        }

        let latest = &versions[0];

        Ok(Some(construct_mod(
            &project.title,
            latest,
            loader,
            game_versions,
        )))
    }

    /// Resolve the mods dependencies.
    pub async fn resolve_dependencies(&self, optional: bool) -> Result<Vec<Mod>, Error> {
        let ids = self
            .dependencies
            .iter()
            .filter(|(dependency_type, _)| match dependency_type {
                DependencyType::Optional => optional,
                _ => true,
            })
            .map(|(_, id)| id);

        let ferinth = Ferinth::default();
        let mut dependencies: Vec<Mod> = vec![];

        for id in ids {
            let version = match id {
                DependencyId::Version(id) => ferinth.get_version(id.as_str()).await?,
                DependencyId::Project(id) => {
                    let versions = ferinth
                        .list_versions_filtered(
                            id.as_str(),
                            Some(&[self.loader.as_str()]),
                            Some(self.game_versions),
                            None,
                        )
                        .await?;
                    versions[0].clone()
                }
            };

            let m = construct_mod(&version.name, &version, self.loader, self.game_versions);
            dependencies.push(m)
        }

        Ok(dependencies)
    }
}
