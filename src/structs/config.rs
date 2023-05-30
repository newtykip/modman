use crate::Error;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write, path::PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigVersions {
    pub minecraft: String,
    pub forge: Option<String>,
    pub fabric: Option<String>,
    pub quilt: Option<String>,
}

/// profile.toml
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub author: String,
    pub version: String,
    pub summary: Option<String>,

    pub versions: ConfigVersions,
}

impl Config {
    pub fn load(path: PathBuf) -> Result<Self, Error> {
        let content = std::fs::read_to_string(path)?;

        Ok(toml::from_str(&content)?)
    }

    pub fn write(&self, path: PathBuf) -> Result<(), Error> {
        let content = toml::to_string(&self)?;

        File::create(path)?.write_all(content.as_bytes())?;

        Ok(())
    }
}
