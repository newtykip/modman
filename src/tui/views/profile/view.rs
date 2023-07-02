use crate::tui::App;
use crossterm::event::KeyCode;
use ratatui::{
    backend::Backend,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct State {
    index: u8,
}

impl Default for State {
    fn default() -> Self {
        Self { index: 0 }
    }
}

pub fn controls(key_code: &KeyCode, _app: &mut App, _state: &mut State) {
    match key_code {
        _ => {}
    }
}

// todo: properly implement
pub fn draw<B: Backend>(frame: &mut Frame<B>, app: &App, _state: &State) {
    let profile = app
        .profile
        .as_ref()
        .expect("profile should have been selected by now");

    frame.render_widget(
        Paragraph::new(profile.author())
            .block(Block::default().borders(Borders::ALL).title(profile.name())),
        frame.size(),
    );
}
