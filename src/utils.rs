use owo_colors::{
    colors::{css::LightBlue, Green, Red, Yellow},
    Color, OwoColorize,
    Stream::Stdout,
};
use std::path::PathBuf;

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
    println!("{}", colour::<Green>(message))
}

pub fn alert(message: &str) {
    println!("{}", colour::<Yellow>(message));
}

pub fn error(message: &str) {
    println!("{}", colour::<Red>(message));
}

pub fn info(message: &str) {
    println!("{}", message.if_supports_color(Stdout, |text| text.bold()));
}

pub fn url(url: &str) -> String {
    underline(&colour::<LightBlue>(url))
}

pub fn modman_dir() -> PathBuf {
    home::home_dir().unwrap().join(".modman")
}

pub fn create_slug(data: &str) -> String {
    data.to_lowercase().replace(' ', "-")
}
