use crate::tui::{
    helpers::{change_view, generate_controls},
    App, View,
};
use crossterm::event::KeyCode;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub struct State {
    index: usize,
}

impl Default for State {
    fn default() -> Self {
        Self { index: 0 }
    }
}

pub fn controls(key_code: &KeyCode, mut app: &mut App, state: &mut State) {
    match key_code {
        // move the cursor up
        KeyCode::Up => {
            if state.index > 0 {
                state.index = state.index.saturating_sub(1);
            }
        }
        // move the cursor down
        KeyCode::Down => {
            if state.index < app.profiles.len().saturating_sub(1) {
                state.index += 1;
            }
        }
        // select the profile
        KeyCode::Enter => {
            app.profile = Some(app.profiles[state.index].clone());
            change_view(&mut app, View::ViewProfile);
        }
        _ => {}
    }
}

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App, state: &State) {
    // split screen into two vertical chunks
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(f.size());

    // profile list on the top
    let profiles = List::new(
        app.profiles
            .iter()
            .enumerate()
            .map(|(i, profile)| {
                let selected = i == state.index;

                ListItem::new(Line::from(vec![
                    // cursor
                    Span::styled(
                        if selected { " > " } else { "   " },
                        Style::default().fg(Color::Green),
                    ),
                    // profile name
                    Span::styled(
                        profile.name(),
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    // profile author
                    Span::styled(
                        format!(" by {}", profile.author()),
                        Style::default().fg(Color::DarkGray),
                    ),
                    // cursor
                    Span::styled(
                        if selected { " < " } else { "   " },
                        Style::default().fg(Color::Green),
                    ),
                ]))
            })
            .collect::<Vec<_>>(),
    )
    .block(
        Block::default()
            .title("Select a Profile")
            .borders(Borders::ALL),
    );

    f.render_widget(profiles, chunks[0]);

    // controls on the bottom
    let controls = generate_controls("up/down to navigate, enter to select");

    f.render_widget(controls, chunks[1]);
}
