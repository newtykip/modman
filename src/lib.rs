mod download;
mod enums;
mod files;
mod sources;
mod utils;

use std::{io::Write, path::PathBuf};

pub use download::Download;
pub use enums::Loader;
pub use files::config;
pub use files::Config;

use enums::{Dependency, Sources};
use serde::Serialize;

pub type Error = Box<dyn std::error::Error>;
type GameVersions = Vec<&'static str>;

/// Represents a Minecraft mod.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Mod {
    /// The name of the mod
    pub name: String,

    /// The mod's filename
    pub filename: String,

    /// A URL to download the mod
    pub url: String,

    /// Dependencies for the mod
    pub dependencies: Vec<Dependency>,

    /// Where the mod is sourced from
    pub source: Sources,

    /// The loader the mod is for
    pub loader: Loader,

    game_versions: GameVersions,
}

impl Mod {
    pub fn write(&self, dir: PathBuf) -> Result<(), Error> {
        let content = toml::to_string(&self)?;
        std::fs::create_dir_all(&dir)?;
        let mut file = std::fs::File::create(dir.join(format!("{}.mm.toml", self.name)))?;

        file.write(content.as_bytes())?;

        Ok(())
    }
}
