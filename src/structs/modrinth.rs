use std::collections::HashSet;

use super::mcmod::{DependencyType, Download, GameVersions, Mod, SearchResult, SupportState};
use crate::{Error, Loader};
use async_recursion::async_recursion;
use ferinth::{
    structures::version::{
        Dependency as FerinthDependency, DependencyType as FerinthDependencyType,
        SupportState as FerinthSupportState,
    },
    Ferinth,
};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use rayon::prelude::*;
use serde_json::Value;

const BASE_URL: &str = "https://api.modrinth.com/v2";

fn map_dependency(value: &FerinthDependency) -> Dependency {
    Dependency {
        version_id: value.version_id.clone(),
        dependency_type: match value.dependency_type {
            FerinthDependencyType::Required => DependencyType::Required,
            FerinthDependencyType::Optional => DependencyType::Optional,
            FerinthDependencyType::Incompatible => DependencyType::Incompatible,
            FerinthDependencyType::Embedded => DependencyType::Embedded,
        },
    }
}

fn map_support_state(value: &FerinthSupportState) -> SupportState {
    match value {
        FerinthSupportState::Required => SupportState::Required,
        FerinthSupportState::Optional => SupportState::Optional,
        FerinthSupportState::Unsupported => SupportState::Unsupported,
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Dependency {
    version_id: Option<String>,
    dependency_type: DependencyType,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct ModrinthMod {
    pub data: Mod,
    dependencies_unresolved: Vec<Dependency>,
}

impl ModrinthMod {
    // todo: build this functionality into ferinth :]
    pub async fn search(
        query: String,
        loader: Loader,
        game_versions: GameVersions<'_>,
    ) -> Result<Vec<SearchResult>, Error> {
        // search on the api
        let results = reqwest::get(format!(
            r#"{}/search?query={}&facets=[["project_type:mod"],["categories:{}"{}],[{}]]"#,
            BASE_URL,
            query,
            loader.to_string(),
            match loader {
                Loader::Quilt => r#","categories:fabric""#,
                _ => "",
            },
            game_versions
                .par_iter()
                .map(|version| format!(r#""versions:{}""#, version))
                .collect::<Vec<_>>()
                .join(",")
        ))
        .await?
        .json::<Value>()
        .await?;

        let results = results["hits"]
            .as_array()
            .unwrap()
            .iter()
            .map(|result| SearchResult {
                name: result["title"].as_str().unwrap().into(),
                id: result["project_id"].as_str().unwrap().into(),
            });

        // fuzzy search for relevant results
        let matcher = SkimMatcherV2::default();

        let scores: Vec<(usize, i64)> = results
            .clone()
            .map(|result| result.name)
            .enumerate()
            .map(|(i, name)| (i, matcher.fuzzy_match(&name, &query).unwrap_or(0)))
            .collect();

        let results: Vec<SearchResult> = results
            .enumerate()
            .filter(|(i, _)| scores[*i].1 != 0)
            .take(5)
            .map(|(_, result)| result)
            .collect();

        Ok(results)
    }

    pub async fn from_project(
        project_id: String,
        loader: Loader,
        game_versions: GameVersions<'_>,
    ) -> Result<ModrinthMod, Error> {
        let client = Ferinth::default();
        let project = client.get_project(&project_id).await?;
        let latest_version = &client
            .list_versions_filtered(
                &project_id,
                Some(&[&loader.to_string()]),
                Some(&game_versions),
                None,
            )
            .await?[0];
        let download = &latest_version.files[0];

        Ok(ModrinthMod {
            data: Mod {
                name: project.title,
                slug: project.slug,
                filename: download.filename.clone(),
                version: latest_version.version_number.clone(),
                client_side: map_support_state(&latest_version.client_side),
                server_side: map_support_state(&latest_version.server_side),
                download: Download {
                    url: download.url.to_string(),
                    sha1: download.hashes.sha1.clone(),
                    sha512: download.hashes.sha512.clone(),
                    file_size: download.size,
                },
            },
            dependencies_unresolved: latest_version
                .dependencies
                .iter()
                .map(map_dependency)
                .collect(),
        })
    }

    #[async_recursion]
    pub async fn resolve_dependencies(
        &self,
        optional: bool,
    ) -> Result<HashSet<ModrinthMod>, Error> {
        let mut resolved: HashSet<ModrinthMod> = HashSet::new();
        let client = Ferinth::default();

        // todo: replace fabric api with quilted fabric api when loader is quilt
        // todo: handle case when dependency.version_id is undefined
        // todo: ensure that mods that have already been installed are ignored
        for dependency in &self.dependencies_unresolved {
            // for now only process the dependency if version_id is defined
            if dependency.version_id.is_none() {
                continue;
            }

            match dependency.dependency_type {
                DependencyType::Optional if !optional => continue,
                DependencyType::Incompatible => continue,
                _ => {}
            }

            let version = client
                .get_version(dependency.version_id.as_ref().unwrap())
                .await?;
            let project = client.get_project(&version.project_id).await?;
            let download = &version.files[0];

            resolved.insert(ModrinthMod {
                data: Mod {
                    name: project.title,
                    slug: project.slug,
                    filename: download.filename.clone(),
                    version: version.version_number.clone(),
                    client_side: map_support_state(&version.client_side),
                    server_side: map_support_state(&version.server_side),
                    download: Download {
                        url: download.url.to_string(),
                        sha1: download.hashes.sha1.clone(),
                        sha512: download.hashes.sha512.clone(),
                        file_size: download.size,
                    },
                },
                dependencies_unresolved: version.dependencies.iter().map(map_dependency).collect(),
            });
        }

        Ok(resolved)
    }
}
