mod download;
mod enums;
mod sources;

pub use download::Download;
pub use enums::Loader;
pub use sources::{curseforge::FromCurse, modrinth::FromModrinth};

use enums::{DependencyType, Sources};

/// Represents a Minecraft mod.
#[derive(Debug)]
pub struct Mod {
    /// The name of the mod
    name: String,

    /// The mod's filename
    filename: String,

    /// A URL to download the mod
    url: String,

    dependencies: Vec<(DependencyType, String)>,

    /// Where the mod is sourced from
    source: Sources,
}
