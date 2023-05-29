use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

use crate::{Error, Toml};

#[derive(Serialize, Deserialize, Debug)]
pub struct Index(HashMap<String, String>);

impl Default for Index {
    fn default() -> Self {
        Self { 0: HashMap::new() }
    }
}

impl Index {
    pub fn append(path: PathBuf, id: String, name: String) -> Result<(), Error> {
        let mut content = toml::from_str::<Self>(std::fs::read_to_string(&path).unwrap().as_str())?;
        content.0.insert(id, name);

        content.write(path)?;

        Ok(())
    }
}
