pub mod structs;

use home::home_dir;
use once_cell::sync::Lazy;
use std::{io, path::PathBuf};
use structs::Profile;

/// modman home directory
pub const MODMAN_DIR: Lazy<PathBuf> = Lazy::new(|| {
    home_dir()
        .expect("home directory should exist")
        .join(".modman")
});

/// generate slug from string
pub fn create_slug(data: &str) -> String {
    data.to_lowercase().replace(' ', "-")
}

/// load all profiles in the home directory
pub fn load_profiles() -> io::Result<Vec<Profile>> {
    let mut profiles = vec![];

    for entry in MODMAN_DIR
        .join("profiles")
        .read_dir()
        .expect("failed to read modman directory")
    {
        let entry = entry.expect("failed to read entry");
        let path = entry.path();

        if path.is_dir() {
            let profile = Profile::load(path)?;

            profiles.push(profile);
        }
    }

    Ok(profiles)
}
