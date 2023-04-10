mod download;
mod enums;
mod sources;

pub use download::Download;
pub use enums::Loader;

use enums::{Dependency, DependencyId, DependencyType, Sources};

type GameVersions = Vec<&'static str>;
type Error = Box<dyn std::error::Error>;

/// Represents a Minecraft mod.
#[derive(Debug, PartialEq, Clone)]
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
