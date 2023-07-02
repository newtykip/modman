use super::Loader;
use crate::{create_slug, MODMAN_DIR};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

#[derive(Clone)]
pub struct Profile {
    config: ProfileConfig,
    pub loader: Option<Loader>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProfileConfig {
    /// the name of the profile
    name: String,

    /// the author associated with the modpack
    author: String,

    /// the current version of the modpack
    version: String,

    /// a short summary of the modpack
    summary: Option<String>,

    /// versions associated with the profile
    versions: ProfileVersions,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProfileVersions {
    pub minecraft: String,
    pub forge: Option<String>,
    pub fabric: Option<String>,
    pub quilt: Option<String>,
}

/// find the path to the profile directory
fn profile_directory(slug: Option<&str>) -> PathBuf {
    let mut path = MODMAN_DIR.join("profiles");

    if let Some(slug) = slug {
        path = path.join(slug);
    }

    path
}

/// figure out which loader is used by the profile's versions
fn resolve_loader(versions: &ProfileVersions) -> Option<Loader> {
    if versions.fabric.is_some() {
        Some(Loader::Fabric)
    } else if versions.forge.is_some() {
        Some(Loader::Forge)
    } else if versions.quilt.is_some() {
        Some(Loader::Quilt)
    } else {
        None
    }
}

impl Profile {
    /// create a new profile
    pub fn create(config: ProfileConfig) -> io::Result<Self> {
        // ensure that the profile has a directory
        let path = profile_directory(Some(&create_slug(&config.name)));

        if !path.exists() {
            fs::create_dir_all(path.join("mods"))?;
        }

        // create the profile.toml file
        let toml_content = toml::to_string(&config).expect("failed to serialize profile config");

        fs::File::create(path)?.write_all(toml_content.as_bytes())?;

        // resolve the loader
        let loader = resolve_loader(&config.versions);

        // todo: create git repository

        Ok(Self { config, loader })
    }

    /// load a profile from a directory
    pub fn load(path: PathBuf) -> io::Result<Self> {
        // parse config
        let config =
            toml::from_str::<ProfileConfig>(&fs::read_to_string(path.join("profile.toml"))?)
                .expect("failed to parse profile.toml");

        // resolve loader
        let loader = resolve_loader(&config.versions);

        Ok(Self { config, loader })
    }

    /// get the name of the profile
    pub fn name(&self) -> String {
        self.config.name.clone()
    }

    /// get the author of the profile
    pub fn author(&self) -> String {
        self.config.author.clone()
    }
}
