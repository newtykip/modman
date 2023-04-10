use crate::{
    enums::{DependencyId, DependencyType, Loader, Sources},
    sources::SearchResult,
    Error, GameVersions, Mod,
};
use furse::{
    structures::{
        file_structs::{File, FileRelationType},
        mod_structs::Mod as FurseMod,
    },
    Furse,
};
use futures_util::Future;
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

fn construct_mod(
    project: FurseMod,
    file: &File,
    furse: Furse,
    loader: Loader,
    game_versions: GameVersions,
) -> CurseMod {
    CurseMod {
        data: Mod {
            name: project.name,
            filename: file.file_name.clone(),
            url: file.download_url.clone().unwrap().to_string(),
            dependencies: file
                .dependencies
                .iter()
                .map(|dependency| {
                    (
                        match dependency.relation_type {
                            FileRelationType::RequiredDependency => DependencyType::Required,
                            FileRelationType::OptionalDependency => DependencyType::Optional,
                            FileRelationType::Include => DependencyType::Incompatible,
                            FileRelationType::EmbeddedLibrary => DependencyType::Embedded,
                            _ => DependencyType::Required,
                        },
                        DependencyId::Project(dependency.mod_id.to_string()),
                    )
                })
                .collect(),
            source: Sources::CurseForge,
            loader,
            game_versions,
        },
        furse,
    }
}

impl Mod {
    /// Get a mod from CurseForge by slug.
    pub async fn from_curseforge(
        slug: &str,
        loader: Loader,
        game_versions: GameVersions,
        furse: Option<Furse>,
        client: Option<Client>,
    ) -> Result<Option<CurseMod>, Error> {
        // todo: implement search into furse - https://github.com/gorilla-devs/furse/issues/5
        let furse = furse.unwrap_or(Furse::new(API_KEY));
        let client = client.unwrap_or(Client::new());

        // find the project
        let search_results = &make_request(
            &client,
            format!("mods/search?gameId=432&pageSize=1&slug={slug}"),
        )
        .await?["data"];
        let projects = search_results.as_array().expect("no results found");

        if projects.len() == 0 {
            return Ok(None);
        }

        // use furse to fetch extra information
        let project_id = projects[0]["id"].as_i64().unwrap() as i32;
        let project = furse.get_mod(project_id).await?;
        let files = furse.get_mod_files(project_id).await?;
        let file = files.iter().find(|file| {
            game_versions.contains(&file.sortable_game_versions[0].game_version_name.as_str())
                && match loader {
                    Loader::Quilt => vec!["Fabric", "Quilt"],
                    Loader::Fabric => vec!["Fabric"],
                    Loader::Forge => vec!["Forge"],
                }
                .contains(&file.sortable_game_versions[1].game_version_name.as_str())
        });

        Ok(if let Some(file) = file {
            Some(construct_mod(project, file, furse, loader, game_versions))
        } else {
            None
        })
    }

    /// Search for mods on Curseforge
    pub fn search_curseforge<'a>(
        query: &'a str,
        loader: Loader,
        game_versions: GameVersions,
        client: Option<Client>,
    ) -> impl Future<Output = Result<Vec<SearchResult>, Error>> + 'a {
        async move {
            let client = client.unwrap_or(Client::new());

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
                    let game_version =
                        game_versions.contains(&index["gameVersion"].as_str().unwrap());

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
    }
}

#[derive(Debug)]
pub struct CurseMod {
    pub data: Mod,
    furse: Furse,
}

impl CurseMod {
    /// Resolve the mods dependencies
    pub async fn resolve_dependencies(&self, optional: bool) -> Result<Vec<CurseMod>, Error> {
        let ids = self
            .data
            .dependencies
            .iter()
            .filter(|(dependency_type, _)| match dependency_type {
                DependencyType::Optional => optional,
                _ => true,
            })
            .map(|(_, id)| id);

        let mut dependencies: Vec<CurseMod> = vec![];

        for id in ids {
            if let DependencyId::Project(id) = id {
                let id = id.parse::<i32>().unwrap();
                let project = self.furse.get_mod(id).await?;
                let files = self.furse.get_mod_files(id).await?;
                let file = files.iter().find(|file| {
                    self.data
                        .game_versions
                        .contains(&file.sortable_game_versions[0].game_version_name.as_str())
                        && match self.data.loader {
                            Loader::Quilt => vec!["Fabric", "Quilt"],
                            Loader::Fabric => vec!["Fabric"],
                            Loader::Forge => vec!["Forge"],
                        }
                        .contains(&file.sortable_game_versions[1].game_version_name.as_str())
                });

                // todo: resolve recursively

                if let Some(file) = file {
                    dependencies.push(construct_mod(
                        project,
                        file,
                        self.furse.clone(),
                        self.data.loader,
                        self.data.game_versions.clone(),
                    ));
                } else {
                    continue;
                }
            } else {
                continue;
            }
        }

        Ok(dependencies)
    }
}
