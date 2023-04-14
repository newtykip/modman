use crate::{files::Config, Error};
use serde::{Deserialize, Serialize};
use std::{io::Write, path::PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct Versions {
    pub minecraft: String,
    pub forge: Option<String>,
    pub fabric: Option<String>,
    pub quilt: Option<String>,
}

impl Config {
    pub fn read(path: PathBuf) -> Result<Config, toml::de::Error> {
        toml::from_str::<Config>(std::fs::read_to_string(path).unwrap().as_str())
    }

    pub fn write(&self, dir: PathBuf) -> Result<(), Error> {
        let content = toml::to_string(&self)?;
        std::fs::create_dir_all(&dir)?;
        let mut file = std::fs::File::create(dir.join("pack.toml"))?;

        file.write(content.as_bytes())?;

        Ok(())
    }
}
