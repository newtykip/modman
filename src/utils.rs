use once_cell::sync::Lazy;
use owo_colors::{
    colors::{css::LightBlue, Green, Red, Yellow},
    Color, OwoColorize,
    Stream::Stdout,
};
use std::path::PathBuf;

pub static MODMAN_DIR: Lazy<PathBuf> = Lazy::new(|| home::home_dir().unwrap().join(".modman"));

pub fn colour<T: Color>(message: &str) -> String {
    message
        .if_supports_color(Stdout, |text| text.fg::<T>())
        .to_string()
}

pub fn bold(message: &str) -> String {
    message
        .if_supports_color(Stdout, |text| text.bold())
        .to_string()
}

pub fn underline(message: &str) -> String {
    message
        .if_supports_color(Stdout, |text| text.underline())
        .to_string()
}

pub fn success(message: &str) {
    println!("{} {}", colour::<Green>(&bold("Success:")), message)
}

pub fn alert(message: &str) {
    println!("{} {}", colour::<Yellow>(&bold("Warn:")), message);
}

pub fn error(message: &str) {
    println!("{} {}", colour::<Red>(&bold("Error:")), message);
}

pub fn info(message: &str) {
    println!("{} {}", bold("Info:"), message);
}

pub fn url(url: &str) -> String {
    underline(&colour::<LightBlue>(url))
}

pub fn create_slug(data: &str) -> String {
    data.to_lowercase().replace(' ', "-")
}
