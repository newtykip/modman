use modman::{create_slug, structs::Profile, MODMAN_DIR};
use std::{fs, io::Write};

#[tauri::command]
#[specta::specta]
pub fn save_profile(profile: Profile) {
    let toml = toml::to_string(&profile).unwrap();
    let path = MODMAN_DIR.join("profiles").join(create_slug(&profile.name));

    fs::create_dir_all(path.join("mods")).unwrap();

    fs::File::create(path.join("profile.toml"))
        .unwrap()
        .write_all(toml.as_bytes())
        .unwrap();
}

#[tauri::command]
#[specta::specta]
pub fn load_profiles() -> Vec<Profile> {
    MODMAN_DIR
        .join("profiles")
        .read_dir()
        .expect("failed to read modman directory")
        .filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap().path())
        .filter(|path: &std::path::PathBuf| path.is_dir())
        .map(Profile::load)
        .filter_map(|profile| profile.ok())
        .collect::<Vec<_>>()
}
