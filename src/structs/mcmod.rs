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

impl<'de> Deserialize<'de> for Side {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match String::deserialize(deserializer)?.as_str() {
            "client" => Ok(Side::Client),
            "server" => Ok(Side::Server),
            "both" => Ok(Side::Both),
            _ => Err(serde::de::Error::custom("invalid side")),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Download {
    pub url: String,
    pub hash_format: String,
    pub hash: String,
}

#[derive(Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Mod {
    pub name: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub slug: String,
    pub filename: String,
    pub version: String,
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
