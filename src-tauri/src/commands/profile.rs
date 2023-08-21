use super::create_slug;
use modman::{structs::Profile, MODMAN_DIR};
use prost::Message;
use std::{fs, io::Write};

/// save a profile object to disk
#[tauri::command]
#[specta::specta]
pub fn save_profile(profile: Profile) {
    let mut buf = vec![];
    profile.encode(&mut buf).unwrap();

    fs::File::create(MODMAN_DIR.join(format!("{}.profile", create_slug(&profile.name))))
        .unwrap()
        .write_all(buf.as_slice())
        .unwrap();
}

/// load all created profiles
#[tauri::command]
#[specta::specta]
pub fn load_profiles() -> Vec<Profile> {
    MODMAN_DIR
        .read_dir()
        .expect("failed to read modman directory")
        .filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().unwrap() == "profile")
        .map(|path| Profile::load(path.file_stem().unwrap().to_str().unwrap()))
        .filter_map(|profile| profile.ok())
        .collect::<Vec<_>>()
}

/// get a profile by its slug
#[tauri::command]
#[specta::specta]
pub fn get_profile(slug: &str) -> Profile {
    // this should never fail, because we only call it on slugs that we know exist
    Profile::load(slug).unwrap()
}
