use std::{fs::File, io::Write, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::Error;

pub mod config;
pub mod index;

pub trait Toml {
    fn read(path: PathBuf) -> Result<Self, toml::de::Error>
    where
        Self: Serialize + for<'de> Deserialize<'de> + Sized,
    {
        toml::from_str::<Self>(std::fs::read_to_string(path).unwrap().as_str())
    }

    fn write(&self, file: PathBuf) -> Result<(), Error>
    where
        Self: Serialize,
    {
        let content = toml::to_string(&self)?;

        File::create(file)?.write(content.as_bytes())?;

        Ok(())
    }
}

impl Toml for config::Config {}
impl Toml for index::Index {}
