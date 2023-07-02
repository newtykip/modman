use super::{App, View};
use ratatui::{
    style::{Color, Modifier, Style},
    widgets::Paragraph,
};

/// change the current view
pub fn change_view(app: &mut App, new_view: View) {
    app.previous_view = Some(app.view);
    app.view = new_view;
}

/// generate controls text
pub fn generate_controls(text: &str) -> Paragraph {
    Paragraph::new(format!("{}, q to go back", text)).style(
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::BOLD),
    )
}
