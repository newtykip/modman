use super::mcmod::{Download, GameVersions, Mod, SearchResult, Side};
use crate::{Error, Loader};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use rayon::prelude::*;
use reqwest::Client;
use serde_json::Value;

const BASE_URL: &str = "https://api.modrinth.com/v2";

#[derive(Debug)]
pub struct ModrinthMod {
    pub data: Mod,
}

impl ModrinthMod {
    // todo: build this functionality into ferinth itself :]
    pub async fn search<'t>(
        query: String,
        loader: Loader,
        game_versions: GameVersions<'t>,
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
                .collect::<Vec<String>>()
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

    pub async fn new<'t>(
        project_id: String,
        loader: Loader,
        game_versions: GameVersions<'t>,
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
                    .collect::<Vec<String>>()
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

        let client_side = project["client_side"].as_str().unwrap();
        let server_side = project["server_side"].as_str().unwrap();

        Ok(ModrinthMod {
            data: Mod {
                name: project["title"].as_str().unwrap().into(),
                slug: project["slug"].as_str().unwrap().into(),
                filename: latest["files"][0]["filename"].as_str().unwrap().into(),
                side: match (client_side, server_side) {
                    ("required", "required") => Side::Both,
                    (_, "required") => Side::Server,
                    ("required", _) => Side::Client,
                    _ => Side::Client,
                },
                download: Download {
                    url: latest["files"][0]["url"].as_str().unwrap().into(),
                    hash_format: "sha1".into(),
                    hash: latest["files"][0]["hashes"]["sha1"]
                        .as_str()
                        .unwrap()
                        .into(),
                },
            },
        })
    }

    // todo: resolve dependencies
}
