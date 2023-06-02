mod structs;

pub mod utils;

use serde::Deserialize;
pub use structs::{Config, ConfigVersions, Mod, ModrinthMod, Profile};

/// Generic error type
pub type Error = Box<dyn std::error::Error>;

/// Supported mod loaders
#[derive(Debug, Deserialize, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Loader {
    Forge,
    Fabric,
    Quilt,
}

impl ToString for Loader {
    fn to_string(&self) -> String {
        match self {
            Loader::Forge => "forge",
            Loader::Fabric => "fabric",
            Loader::Quilt => "quilt",
        }
        .to_string()
    }
}
