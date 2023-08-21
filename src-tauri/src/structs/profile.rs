use crate::MODMAN_DIR;
use prost::{DecodeError, Enumeration, Message};
use serde::{Deserialize, Serialize};
use specta::Type;
use std::fs;

#[derive(Serialize, Deserialize, Debug, Enumeration, Type)]
pub enum Loader {
    Forge = 0,
    Fabric = 1,
    Quilt = 2,
}

#[derive(Serialize, Deserialize, Message, Type)]
pub struct Profile {
    /// the name of the profile
    #[prost(string)]
    pub name: String,

    /// the author associated with the modpack
    #[prost(string)]
    pub author: String,

    /// the current version of the modpack
    #[prost(string)]
    pub version: String,

    /// The version of Minecraft the profile is for
    #[prost(string)]
    pub minecraft_version: String,

    /// The loader to use for the profile
    #[prost(enumeration = "Loader")]
    pub loader: i32,

    /// The version of the loader to use for the profile
    #[prost(string)]
    pub loader_version: String,
}

impl Profile {
    pub fn load(slug: &str) -> Result<Self, DecodeError> {
        Profile::decode(
            fs::read(MODMAN_DIR.join(format!("{slug}.profile")))
                .expect(&format!("failed to read {slug}.profile"))
                .as_slice(),
        )
    }
}
