use crate::{utils::modman_dir, ConfigVersions, Error, Loader};

use super::{mcmod::Mod, Config};
use git2::Repository;
use std::{fmt::Display, fs, path::PathBuf};

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

pub struct Profile {
    pub config: Config,
    pub path: PathBuf,
    pub repo: Option<Repository>,
    pub loader: Loader,
}

impl Profile {
    /// Convert a name to an id
    pub fn name_to_id(name: &str) -> String {
        name.to_lowercase().replace(' ', "-")
    }

    /// Get the directory of the profiles
    pub fn directory(id: Option<&str>) -> PathBuf {
        let profiles = modman_dir().join("profiles");

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

    /// Create a new profile
    pub fn new(config: Config) -> Result<Self, Error> {
        let path = Profile::directory(Some(&Self::name_to_id(&config.name)));

        // ensure that the profile's mods directory exists
        if !path.exists() {
            std::fs::create_dir_all(path.join("mods"))?;
        }

        // create the profile.toml file
        config.write(path.join("profile.toml"))?;

        // resolve the loader
        let loader = determine_loader(&config.versions).unwrap();

        // create a git repository
        let repo = match Repository::init(&path) {
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

    /// Load a profile
    pub fn load(id: &str) -> Result<Self, Error> {
        let path = Profile::directory(Some(id));
        let repo = match Repository::open(&path) {
            Ok(repo) => Some(repo),
            Err(_) => None,
        };

        let config = Config::load(path.join("profile.toml"))?;
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
        let profile_directory = Profile::directory(None);

        Ok(fs::read_dir(profile_directory)?
            .map(|entry| {
                let path = entry.unwrap().path();
                let config = Config::load(path.join("profile.toml")).unwrap();
                let repo = match Repository::open(&path) {
                    Ok(repo) => Some(repo),
                    Err(_) => None,
                };
                let loader = determine_loader(&config.versions).unwrap();

                Self {
                    config,
                    path,
                    repo,
                    loader,
                }
            })
            .collect())
    }

    /// Load the selected profile
    pub fn load_selected() -> Result<Self, Error> {
        let selected = fs::read_to_string(modman_dir().join(".selected"))?;

        Self::load(&selected)
    }

    pub fn select(&self) -> Result<(), Error> {
        fs::write(
            modman_dir().join(".selected"),
            Profile::name_to_id(&self.config.name),
        )?;

        Ok(())
    }

    pub fn add_mod(&self, mcmod: Mod) -> Result<(), Error> {
        mcmod.write(
            self.path
                .join("mods")
                .join(format!("{}.mm.toml", mcmod.slug)),
        )?;

        Ok(())
    }
}

impl Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.config.name)
    }
}
