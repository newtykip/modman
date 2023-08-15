#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum Loader {
    Fabric,
    Forge,
    Quilt,
    #[default]
    Unknown,
}

impl ToString for Loader {
    fn to_string(&self) -> String {
        match self {
            Loader::Fabric => "fabric",
            Loader::Forge => "forge",
            Loader::Quilt => "quilt",
            Loader::Unknown => "unknown",
        }
        .to_string()
    }
}
