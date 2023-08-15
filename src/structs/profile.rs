use super::Loader;
use crate::{create_slug, MODMAN_DIR};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Profile {
    /// the name of the profile
    pub name: String,

    /// the author associated with the modpack
    pub author: String,

    /// the current version of the modpack
    pub version: String,

    /// a short summary of the modpack
    pub summary: Option<String>,

    /// versions associated with the profile
    pub versions: ProfileVersions,

    #[serde(skip)]
    pub loader: Loader,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ProfileVersions {
    pub minecraft: String,
    pub forge: Option<String>,
    pub fabric: Option<String>,
    pub quilt: Option<String>,
}

impl ProfileVersions {
    pub fn get_version(&self, loader: Option<Loader>) -> String {
        match loader {
            Some(Loader::Forge) => self.forge.clone().unwrap(),
            Some(Loader::Fabric) => self.fabric.clone().unwrap(),
            Some(Loader::Quilt) => self.quilt.clone().unwrap(),
            _ => self.minecraft.clone(),
        }
    }
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
fn resolve_loader(versions: &ProfileVersions) -> Loader {
    if versions.fabric.is_some() {
        Loader::Fabric
    } else if versions.forge.is_some() {
        Loader::Forge
    } else if versions.quilt.is_some() {
        Loader::Quilt
    } else {
        Loader::Unknown
    }
}

impl Profile {
    /// create a new profile
    pub fn create(
        name: String,
        author: String,
        version: String,
        versions: ProfileVersions,
        summary: Option<String>,
    ) -> io::Result<Self> {
        // ensure that the profile has a directory
        let path = profile_directory(Some(&create_slug(&name)));

        if !path.exists() {
            fs::create_dir_all(path.join("mods"))?;
        }

        // todo: create git repository

        // create the profile
        let profile = Self {
            name,
            author,
            version,
            summary,
            loader: resolve_loader(&versions),
            versions,
        };

        // create the profile.toml file
        let toml_content = toml::to_string(&profile).expect("failed to serialize profile config");

        fs::File::create(path)?.write_all(toml_content.as_bytes())?;

        Ok(profile)
    }

    /// load a profile from a directory
    pub fn load(path: PathBuf) -> Result<Self, toml::de::Error> {
        let mut profile = toml::from_str::<Profile>(
            &fs::read_to_string(path.join("profile.toml")).expect("failed to read profile.toml"),
        )?;

        profile.loader = resolve_loader(&profile.versions);

        Ok(profile)
    }
}
