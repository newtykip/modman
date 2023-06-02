use std::collections::HashSet;

use super::mcmod::{DependencyType, Download, GameVersions, Mod, SearchResult, Side};
use crate::{Error, Loader};
use async_recursion::async_recursion;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use rayon::prelude::*;
use reqwest::Client;
use serde_json::Value;

const BASE_URL: &str = "https://api.modrinth.com/v2";

fn map_dependency(value: &Value) -> ModrinthDependency {
    ModrinthDependency {
        version_id: value["version_id"].as_str().map(|s| s.to_string()),
        dependency_type: match value["dependency_type"].as_str().unwrap() {
            "required" => DependencyType::Required,
            "optional" => DependencyType::Optional,
            "incompatible" => DependencyType::Incompatible,
            "embedded" => DependencyType::Embedded,
            _ => unreachable!(),
        },
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct ModrinthDependency {
    version_id: Option<String>,
    dependency_type: DependencyType,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct ModrinthMod {
    pub data: Mod,
    dependencies_unresolved: Vec<ModrinthDependency>,
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
        let client = Client::new();

        // get the latest version
        let versions = client
            .get(format!(
                "{}/project/{}/version?loaders=[{}{}]&game_versions=[{}]",
                BASE_URL,
                project_id,
                loader.to_string(),
                match loader {
                    Loader::Quilt => r#","fabric""#,
                    _ => "",
                },
                game_versions
                    .par_iter()
                    .map(|version| format!(r#""{}""#, version))
                    .collect::<Vec<_>>()
                    .join(",")
            ))
            .send()
            .await?
            .json::<Value>()
            .await?;

        let latest = &versions.as_array().unwrap()[0];

        // lookup project data
        let project = client
            .get(format!("{}/project/{}", BASE_URL, project_id))
            .send()
            .await?
            .json::<Value>()
            .await?;

        let side = match (
            project["client_side"].as_str().unwrap(),
            project["server_side"].as_str().unwrap(),
        ) {
            ("required", "required") => Side::Both,
            (_, "required") => Side::Server,
            ("required", _) => Side::Client,
            _ => Side::Client,
        };

        Ok(ModrinthMod {
            data: Mod {
                name: project["title"].as_str().unwrap().into(),
                slug: project["slug"].as_str().unwrap().into(),
                filename: latest["files"][0]["filename"].as_str().unwrap().into(),
                version: latest["version_number"].as_str().unwrap().into(),
                side,
                download: Download {
                    url: latest["files"][0]["url"].as_str().unwrap().into(),
                    hash_format: "sha1".into(),
                    hash: latest["files"][0]["hashes"]["sha1"]
                        .as_str()
                        .unwrap()
                        .into(),
                },
            },
            dependencies_unresolved: latest["dependencies"]
                .as_array()
                .unwrap()
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

        // todo: replace fabric api with quilted fabric api when loader is quilt
        // todo: handle case when dependency.version_id is undefined
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

            let version = reqwest::get(format!(
                "{}/version/{}",
                BASE_URL,
                dependency.version_id.as_ref().unwrap()
            ))
            .await?
            .json::<Value>()
            .await?;

            let project = reqwest::get(format!(
                "{}/project/{}",
                BASE_URL,
                version["project_id"].as_str().unwrap()
            ))
            .await?
            .json::<Value>()
            .await?;

            let side = match (
                project["client_side"].as_str().unwrap(),
                project["server_side"].as_str().unwrap(),
            ) {
                ("required", "required") => Side::Both,
                (_, "required") => Side::Server,
                ("required", _) => Side::Client,
                _ => Side::Client,
            };

            resolved.insert(ModrinthMod {
                data: Mod {
                    name: project["title"].as_str().unwrap().into(),
                    slug: project["slug"].as_str().unwrap().into(),
                    filename: version["files"][0]["filename"].as_str().unwrap().into(),
                    version: version["version_number"].as_str().unwrap().into(),
                    side,
                    download: Download {
                        url: version["files"][0]["url"].as_str().unwrap().into(),
                        hash_format: "sha1".into(),
                        hash: version["files"][0]["hashes"]["sha1"]
                            .as_str()
                            .unwrap()
                            .into(),
                    },
                },
                dependencies_unresolved: version["dependencies"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(map_dependency)
                    .collect(),
            });
        }

        Ok(resolved)
    }
}
