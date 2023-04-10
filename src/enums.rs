use std::str::FromStr;

/// Sources that modman can fetch mods from.
#[derive(Debug, PartialEq, Clone)]
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
#[derive(Debug, PartialEq, Clone)]
pub enum DependencyType {
    Optional,
    Required,
    Incompatible,
    Embedded,
}

#[derive(Debug, PartialEq, Clone)]
pub enum DependencyId {
    Project(String),
    Version(String),
}

impl DependencyId {
    pub fn parse<T: FromStr>(&self) -> Result<T, <T as FromStr>::Err> {
        match self {
            DependencyId::Project(x) => x.parse::<T>(),
            DependencyId::Version(x) => x.parse::<T>(),
        }
    }
}

pub type Dependency = (DependencyType, DependencyId);

/// All of the supported mod loaders.
#[derive(Debug, Clone, Copy, PartialEq)]
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
