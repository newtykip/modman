use serde::{Deserialize, Serialize};
use specta::Type;
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Type)]
#[serde(rename_all = "lowercase")]
pub enum Loader {
    Forge,
    Fabric,
    Quilt,
}

#[derive(Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    /// the name of the profile
    pub name: String,

    /// the author associated with the modpack
    pub author: String,

    /// the current version of the modpack
    pub version: String,

    /// a short summary of the modpack
    pub summary: Option<String>,

    /// The version of Minecraft the profile is for
    pub minecraft_version: String,

    /// The loader to use for the profile
    pub loader: Loader,

    pub loader_version: String,
}

impl Profile {
    pub fn load(path: PathBuf) -> Result<Self, toml::de::Error> {
        toml::from_str::<Profile>(
            &fs::read_to_string(path.join("profile.toml")).expect("failed to read profile.toml"),
        )
    }
}
