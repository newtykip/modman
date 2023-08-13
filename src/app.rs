use crate::structs::Profile;
use crate::views::Views;
use crate::MODMAN_DIR;

#[derive(Clone)]
pub struct App {
    pub view: Views,
    pub profiles: Vec<Profile>,
    pub selected: Option<Profile>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            view: Views::SelectProfile,
            profiles: MODMAN_DIR
                .join("profiles")
                .read_dir()
                .expect("failed to read modman directory")
                .filter(|entry| entry.is_ok())
                .map(|entry| entry.unwrap().path())
                .filter(|path: &std::path::PathBuf| path.is_dir())
                .map(Profile::load)
                .filter_map(|profile| profile.ok())
                .collect::<Vec<_>>(),
            selected: None,
        }
    }
}
