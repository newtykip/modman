/// Sources that modman can fetch mods from.
#[derive(Debug)]
pub enum Sources {
    Modrinth,
    CurseForge,
}

impl ToString for Sources {
    fn to_string(&self) -> String {
        match self {
            Sources::CurseForge => "CurseForge",
            Sources::Modrinth => "Modrinth",
        }
        .to_string()
    }
}

/// Types of mod dependencies.
#[derive(Debug)]
pub enum DependencyType {
    Optional,
    Required,
    Incompatible,
    Embedded,
}

/// All of the supported mod loaders.
#[derive(Debug, Clone, Copy)]
pub enum Loader {
    Forge,
    Fabric,
    Quilt,
}

impl Loader {
    pub fn as_str(&self) -> &str {
        match self {
            Loader::Forge => "forge",
            Loader::Fabric => "fabric",
            Loader::Quilt => "quilt",
        }
    }

    pub fn curseforge(&self) -> Vec<u8> {
        match self {
            Loader::Forge => vec![1],
            Loader::Fabric => vec![4],
            Loader::Quilt => vec![4, 5],
        }
    }
}
