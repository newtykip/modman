use super::mcmod::Mod;
use crate::{
    utils::{create_slug, MODMAN_DIR},
    Error, Loader,
};
use git2::{Repository, RepositoryInitOptions};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

static SELECTED_PATH: Lazy<PathBuf> = Lazy::new(|| MODMAN_DIR.join(".selected"));

fn determine_loader(versions: &ConfigVersions) -> Option<Loader> {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigVersions {
    pub minecraft: String,
    pub forge: Option<String>,
    pub fabric: Option<String>,
    pub quilt: Option<String>,
}

/// profile.toml
#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileConfig {
    pub name: String,
    pub author: String,
    pub version: String,
    pub summary: Option<String>,

    pub versions: ConfigVersions,
}

impl ProfileConfig {
    pub fn write(&self, path: PathBuf) -> Result<(), Error> {
        let content = toml::to_string(&self)?;

        File::create(path)?.write_all(content.as_bytes())?;

        Ok(())
    }

    pub fn load(path: PathBuf) -> Result<Self, Error> {
        let content = std::fs::read_to_string(path)?;

        Ok(toml::from_str(&content)?)
    }
}

pub struct Profile {
    pub config: ProfileConfig,
    pub path: PathBuf,
    pub repo: Option<Repository>,
    pub loader: Loader,
}

impl PartialEq for Profile {
    fn eq(&self, other: &Self) -> bool {
        self.config.name == other.config.name
    }
}

impl Profile {
    // * create

    /// Create a new profile
    pub fn new(config: ProfileConfig) -> Result<Self, Error> {
        let path = Profile::directory(Some(&create_slug(&config.name)));

        // ensure that the profile's mods directory exists
        if !path.exists() {
            std::fs::create_dir_all(path.join("mods"))?;
        }

        // create the profile.toml file
        config.write(path.join("profile.toml"))?;

        // resolve the loader
        let loader = determine_loader(&config.versions).unwrap();

        // create a git repository
        let repo =
            match Repository::init_opts(&path, RepositoryInitOptions::new().initial_head("main")) {
                Ok(repo) => Some(repo),
                Err(_) => None,
            };

        Ok(Self {
            config,
            path,
            repo,
            loader,
        })
    }

    // * load

    /// Load a profile
    pub fn load(id: &str) -> Result<Self, Error> {
        let path = Profile::directory(Some(id));
        let repo = match Repository::open(&path) {
            Ok(repo) => Some(repo),
            Err(_) => None,
        };

        let config = ProfileConfig::load(path.join("profile.toml"))?;
        let loader = determine_loader(&config.versions).unwrap();

        Ok(Self {
            config,
            path,
            repo,
            loader,
        })
    }

    /// Load all profiles
    pub fn load_all() -> Result<Vec<Self>, Error> {
        Ok(Profile::used_ids()?
            .iter()
            .map(|id| Profile::load(id).unwrap())
            .collect::<Vec<_>>())
    }

    /// Load the selected profile
    pub fn get_selected() -> Result<Self, Error> {
        let selected = fs::read_to_string(SELECTED_PATH.clone())?;

        Self::load(&selected)
    }

    // * actions

    /// Mark the profile as selected
    pub fn select(&self) -> Result<(), Error> {
        fs::write(MODMAN_DIR.join(".selected"), create_slug(&self.config.name))?;

        Ok(())
    }

    /// Add a mod to the profile
    pub fn add_mod(&self, mcmod: &Mod) -> Result<(), Error> {
        mcmod.write(
            self.path
                .join("mods")
                .join(format!("{}.mm.toml", mcmod.slug)),
        )?;

        Ok(())
    }

    /// Get all of the mods associated witgh the profile
    pub fn get_mods(&self) -> Result<Vec<Mod>, Error> {
        Ok(fs::read_dir(self.path.join("mods"))?
            .map(|entry| {
                let path = entry.unwrap().path();
                let contents = fs::read_to_string(path).unwrap();

                toml::from_str(&contents).unwrap()
            })
            .collect())
    }

    /// Delete the profile
    pub fn delete(&self) -> Result<(), Error> {
        fs::remove_dir_all(&self.path)?;
        Ok(())
    }

    // pub async fn create_modrinth(&self) -> Result<(), Error> {
    //     Ok(())
    // }

    // * utilities

    /// Is the profile selected?
    pub fn is_selected(&self) -> Result<bool, Error> {
        let contents = fs::read_to_string(SELECTED_PATH.clone())?;
        Ok(contents == create_slug(&self.config.name))
    }

    /// Get the directory of the profiles
    fn directory(id: Option<&str>) -> PathBuf {
        let profiles = SELECTED_PATH.join("profiles");

        if let Some(id) = id {
            profiles.join(id)
        } else {
            profiles
        }
    }

    /// List all of the used profile ids
    pub fn used_ids() -> Result<Vec<String>, Error> {
        let profiles = Profile::directory(None);

        Ok(if !profiles.exists() {
            vec![]
        } else {
            fs::read_dir(profiles)?
                .map(|p| p.unwrap().path()) // get all of the paths in the profiles directory
                .filter(|p| p.is_dir()) // filter out all of the files
                .map(|f| f.file_name().unwrap().to_str().unwrap().into()) // get the name of the directory
                .collect()
        })
    }
}

impl Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.config.name)
    }
}
