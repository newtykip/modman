use once_cell::sync::Lazy;
use std::path::PathBuf;

pub mod structs;

pub static MODMAN_DIR: Lazy<PathBuf> = Lazy::new(|| {
    home::home_dir()
        .expect("home directory should exist")
        .join(".modman")
});
