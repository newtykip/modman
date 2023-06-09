use crate::{utils::MODMAN_DIR, Error};
use once_cell::sync::Lazy;
use phf::{phf_map, Map};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::Write, path::PathBuf};

pub enum ValueType {
    String,
}

pub static CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| MODMAN_DIR.join("config.toml"));
pub static CONFIG_TYPES: Map<&'static str, ValueType> = phf_map! {
    "modrinth_token" => ValueType::String,
    "curseforge_token" => ValueType::String,
};
pub static CONFIG_CENSOR: Map<&'static str, bool> = phf_map! {
    "modrinth_token" => true,
    "curseforge_token" => true,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config(pub HashMap<String, Option<String>>);

impl Config {
    pub fn load() -> Result<Self, Error> {
        let content = std::fs::read_to_string(CONFIG_PATH.clone())?;
        Ok(toml::from_str(&content)?)
    }

    pub fn save(&self) -> Result<(), Error> {
        let content = toml::to_string(&self)?;
        File::create(CONFIG_PATH.clone())?.write_all(content.as_bytes())?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut values = HashMap::new();

        for key in CONFIG_TYPES.keys() {
            values.insert(key.to_string(), None);
        }

        Self(values)
    }
}
