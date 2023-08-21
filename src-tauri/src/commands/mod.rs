mod profile;

pub use profile::*;

/// generate profile slug from string
#[tauri::command]
#[specta::specta]
pub fn create_slug(data: &str) -> String {
    data.to_lowercase().replace(' ', "-")
}
