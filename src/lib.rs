pub use app::App;
use home::home_dir;
use once_cell::sync::Lazy;
use std::path::PathBuf;
use views::Views;

mod app;
pub mod structs;
pub mod views;

/// modman home directory
pub static MODMAN_DIR: Lazy<PathBuf> = Lazy::new(|| {
    home_dir()
        .expect("home directory should exist")
        .join(".modman")
});

/// generate slug from string
pub fn create_slug(data: &str) -> String {
    data.to_lowercase().replace(' ', "-")
}

/// change the current view
pub fn change_view(app: &mut App, view: Views) {
    app.view = view;
}
