use crate::Error;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, fs::File, hash::Hash, io::Write, path::PathBuf};

pub type GameVersions<'t> = Vec<&'t str>;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum SupportState {
    Required,
    Optional,
    Unsupported,
}

impl ToString for SupportState {
    fn to_string(&self) -> String {
        match self {
            SupportState::Required => "required",
            SupportState::Optional => "optional",
            SupportState::Unsupported => "unsupported",
        }
        .to_string()
    }
}

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

#[derive(Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Download {
    pub url: String,
    pub sha1: String,
    pub sha512: String,
    pub file_size: usize,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Mod {
    pub name: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub slug: String,
    pub filename: String,
    pub version: String,
    pub client_side: SupportState,
    pub server_side: SupportState,

    pub download: Download,
}

impl Mod {
    pub fn write(&self, path: PathBuf) -> Result<(), Error> {
        let content = toml::to_string(&self)?;

        File::create(path)?.write_all(content.as_bytes())?;

        Ok(())
    }
}
