use owo_colors::{colors::Green, Color, OwoColorize, Stream::Stdout};

fn colour<T: Color>(message: &str) -> String {
    message
        .if_supports_color(Stdout, |text| text.fg::<T>())
        .to_string()
}

pub fn success(message: &str) {
    println!("{}", colour::<Green>(message))
}
