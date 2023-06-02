use std::{fmt::Display, fs::File, io::Write, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::Error;

pub type GameVersions<'t> = Vec<&'t str>;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum DependencyType {
    Required,
    Optional,
    Incompatible,
    Embedded,
}

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

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum ModSide {
    Client,
    Server,
    Both,
}

impl Serialize for ModSide {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            ModSide::Client => serializer.serialize_str("client"),
            ModSide::Server => serializer.serialize_str("server"),
            ModSide::Both => serializer.serialize_str("both"),
        }
    }
}

impl<'de> Deserialize<'de> for ModSide {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match String::deserialize(deserializer)?.as_str() {
            "client" => Ok(ModSide::Client),
            "server" => Ok(ModSide::Server),
            "both" => Ok(ModSide::Both),
            _ => Err(serde::de::Error::custom("invalid side")),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Download {
    pub url: String,
    pub hash_format: String,
    pub sha1: String,
    pub sha512: String,
    pub file_size: u64,
}

#[derive(Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Mod {
    pub name: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub slug: String,
    pub filename: String,
    pub version: String,
    pub side: ModSide,

    pub download: Download,
}

impl Mod {
    pub fn write(&self, path: PathBuf) -> Result<(), Error> {
        let content = toml::to_string(&self)?;

        File::create(path)?.write_all(content.as_bytes())?;

        Ok(())
    }
}
