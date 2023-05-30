use crate::{utils::modman_dir, Error};

use super::Config;
use std::{fmt::Display, fs, path::PathBuf};

pub struct Profile {
    pub config: Config,
}

impl Profile {
    /// Convert a name to an id
    pub fn name_to_id(name: &str) -> String {
        name.to_lowercase().replace(" ", "-")
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
                .map(|f| f.file_name().unwrap().to_str().unwrap().to_string()) // get the name of the directory
                .collect()
        })
    }

    /// Create a new profile
    pub fn new(config: Config) -> Result<Self, Error> {
        let profile_directory = Profile::directory(Some(&Self::name_to_id(&config.name)));

        // ensure that the profile's directory exists
        if !profile_directory.exists() {
            std::fs::create_dir_all(profile_directory.clone()).unwrap();
        }

        // create the profile.toml file
        config.write(profile_directory.join("profile.toml"))?;

        // select the profile

        Ok(Self { config })
    }

    /// Load a profile
    pub fn load(id: &str) -> Result<Self, Error> {
        let profile_directory = Profile::directory(Some(id));

        Ok(Self {
            config: Config::load(profile_directory.join("profile.toml"))?,
        })
    }

    /// Load all profiles
    pub fn load_all() -> Result<Vec<Self>, Error> {
        let profile_directory = Profile::directory(None);

        Ok(fs::read_dir(profile_directory)
            .unwrap()
            .map(|entry| {
                let path = entry.unwrap().path();
                let config = Config::load(path.join("profile.toml")).unwrap();
                Self { config }
            })
            .collect())
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
