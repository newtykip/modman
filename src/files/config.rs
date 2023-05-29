use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigVersions {
    pub minecraft: String,
    pub forge: Option<String>,
    pub fabric: Option<String>,
    pub quilt: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub author: String,
    pub version: String,
    pub versions: ConfigVersions,
}
