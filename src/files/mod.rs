use serde::{Deserialize, Serialize};

pub mod config;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub author: String,
    pub version: String,
    pub versions: config::Versions,
}
