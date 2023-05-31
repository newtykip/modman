mod structs;

pub mod utils;

pub use structs::{Config, ConfigVersions, ModrinthMod, Profile};

/// Generic error type
pub type Error = Box<dyn std::error::Error>;

/// Supported mod loaders
#[derive(PartialEq, Clone, Copy)]
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
