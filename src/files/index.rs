use crate::{Error, Toml};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct Index(
    /// The index file's contents.
    /// Also holds a key called "path" which is the path to the index file
    pub HashMap<String, String>,
);

impl Index {
    pub fn new(path: PathBuf) -> Self {
        let mut map = HashMap::new();
        map.insert("path".to_string(), path.to_str().unwrap().to_string());

        Self { 0: map }
    }

    pub fn append(&self, id: String, name: String) -> Result<(), Error> {
        let path = self.0.get("path").unwrap();
        let mut content = toml::from_str::<Self>(std::fs::read_to_string(path).unwrap().as_str())?;

        content.0.insert(id, name);
        content.write(PathBuf::from(path))?;

        Ok(())
    }
}
