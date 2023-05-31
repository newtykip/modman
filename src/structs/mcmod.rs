use std::{fmt::Display, fs::File, io::Write, path::PathBuf};

use serde::Serialize;

use crate::Error;

pub type GameVersions<'t> = Vec<&'t str>;

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub name: String,
    pub id: String,
}

impl Display for SearchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug)]
pub enum Side {
    Client,
    Server,
    Both,
}

impl Serialize for Side {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Side::Client => serializer.serialize_str("client"),
            Side::Server => serializer.serialize_str("server"),
            Side::Both => serializer.serialize_str("both"),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Download {
    pub url: String,
    pub hash_format: String,
    pub hash: String,
}

#[derive(Debug, Serialize)]
pub struct Mod {
    pub name: String,
    #[serde(skip_serializing)]
    pub slug: String,
    pub filename: String,
    pub side: Side,

    pub download: Download,
}

impl Mod {
    pub fn write(&self, path: PathBuf) -> Result<(), Error> {
        let content = toml::to_string(&self)?;

        File::create(path)?.write_all(content.as_bytes())?;

        Ok(())
    }
}
