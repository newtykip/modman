use std::path::PathBuf;

use owo_colors::{
    colors::{Green, Red, Yellow},
    Color, OwoColorize,
    Stream::Stdout,
};

fn colour<T: Color>(message: &str) -> String {
    message
        .if_supports_color(Stdout, |text| text.fg::<T>())
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

pub fn modman_dir() -> PathBuf {
    home::home_dir().unwrap().join(".modman")
}
