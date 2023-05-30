use crate::{utils::modman_dir, Error};

use super::Config;
use git2::Repository;
use std::{fmt::Display, fs, path::PathBuf};

pub struct Profile {
    pub config: Config,
    pub path: PathBuf,
    pub repo: Option<Repository>,
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
        let directory = Profile::directory(Some(&Self::name_to_id(&config.name)));

        // ensure that the profile's directory exists
        if !directory.exists() {
            std::fs::create_dir_all(directory.clone()).unwrap();
        }

        // create the profile.toml file
        config.write(directory.join("profile.toml"))?;

        // create a git repository
        let repository = match Repository::init(&directory) {
            Ok(repo) => Some(repo),
            Err(_) => None,
        };

        Ok(Self {
            config,
            path: directory,
            repo: repository,
        })
    }

    /// Load a profile
    pub fn load(id: &str) -> Result<Self, Error> {
        let directory = Profile::directory(Some(id));
        let repository = match Repository::open(&directory) {
            Ok(repo) => Some(repo),
            Err(_) => None,
        };

        Ok(Self {
            config: Config::load(directory.join("profile.toml"))?,
            path: directory,
            repo: repository,
        })
    }

    /// Load all profiles
    pub fn load_all() -> Result<Vec<Self>, Error> {
        let profile_directory = Profile::directory(None);

        Ok(fs::read_dir(profile_directory)
            .unwrap()
            .map(|entry| {
                let directory = entry.unwrap().path();
                let config = Config::load(directory.join("profile.toml")).unwrap();
                let repository = match Repository::open(&directory) {
                    Ok(repo) => Some(repo),
                    Err(_) => None,
                };

                Self {
                    config,
                    path: directory,
                    repo: repository,
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
}

impl Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.config.name)
    }
}
