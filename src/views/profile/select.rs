use crate::{change_view, structs::Profile, views::Views, App};
use crossterm::event::KeyCode;
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListItem, ListState, Paragraph},
    Frame,
};

// todo: make this scale to the terminal window size
const HORIZONTAL_CHUNKS: usize = 3;

fn chunk_profiles(app: &App) -> impl Iterator<Item = Vec<Option<&Profile>>> {
    (0..HORIZONTAL_CHUNKS).map(|i| {
        app.profiles
            .iter()
            .skip(i)
            .step_by(HORIZONTAL_CHUNKS)
            .map(Some)
            .collect()
    })
}

#[derive(Default)]
pub struct State {
    horizontal: u8,
    vertical: usize,
}

pub fn draw<B: Backend>(frame: &mut Frame<B>, app: &App, state: &State) {
    // divide it into chunks
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Max(app.profiles.len() as u16),
            Constraint::Length(1),
        ])
        .split(frame.size());

    // render the header
    let header = Paragraph::new(vec![
        Line::from(Span::styled("modman", Style::default().fg(Color::Green))),
        Line::from("Please select a profile"),
    ])
    .alignment(Alignment::Center);

    frame.render_widget(header, chunks[0]);

    // render the profiles
    let list_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(chunks[1]);

    let max_len = chunk_profiles(app).map(|col| col.len()).max().unwrap_or(0);

    for (i, mut profiles) in chunk_profiles(app).enumerate() {
        // make sure they are the same length
        for _ in 0..(max_len - profiles.len()) {
            profiles.push(None);
        }

        let mut list_state = ListState::default();
        list_state.select(Some(state.vertical));

        let list = List::new(
            profiles
                .iter()
                .enumerate()
                .map(|(j, profile)| {
                    let selected = i as u8 == state.horizontal && j == state.vertical;

                    if let Some(profile) = profile {
                        ListItem::new(Line::from(vec![
                            // cursor
                            Span::styled(
                                if selected { " > " } else { "   " },
                                Style::default().fg(Color::Green),
                            ),
                            // profile name
                            Span::styled(
                                profile.name.clone(),
                                Style::default().add_modifier(Modifier::BOLD),
                            ),
                            // profile author
                            Span::styled(
                                format!(" by {}", profile.author),
                                Style::default().fg(Color::DarkGray),
                            ),
                            // cursor
                            Span::styled(
                                if selected { " < " } else { "   " },
                                Style::default().fg(Color::Green),
                            ),
                        ]))
                    } else {
                        ListItem::new("")
                    }
                })
                .collect::<Vec<_>>(),
        );

        frame.render_stateful_widget(list, list_chunks[i], &mut list_state);
    }
}

pub fn controls(key_code: KeyCode, app: &mut App, state: &mut State) {
    let profile_count = app.profiles.len();
    let row_count = (profile_count + HORIZONTAL_CHUNKS - 1) / HORIZONTAL_CHUNKS;

    match key_code {
        // move the cursor up
        KeyCode::Up | KeyCode::Char('w') => state.vertical = state.vertical.saturating_sub(1),
        // move the cursor down
        KeyCode::Down | KeyCode::Char('s') => {
            state.vertical = (state.vertical + 1).min(row_count - 1);

            // stop the horizontal index from going out of bounds
            state.horizontal = state.horizontal.min(if state.vertical == row_count - 1 {
                let remainder = ((profile_count + HORIZONTAL_CHUNKS - 1) as f32)
                    / HORIZONTAL_CHUNKS as f32
                    % 1.;
                let mut min = HORIZONTAL_CHUNKS - 1;

                for i in 1..HORIZONTAL_CHUNKS {
                    if remainder < i as f32 / HORIZONTAL_CHUNKS as f32 {
                        min = i;
                        break;
                    }
                }

                min
            } else {
                HORIZONTAL_CHUNKS - 1
            } as u8);
        }
        // move the cursor left
        KeyCode::Left | KeyCode::Char('a') => state.horizontal = state.horizontal.saturating_sub(1),
        // move the cursor right
        KeyCode::Right | KeyCode::Char('d') => {
            // stop the horizontal index from going out of bounds
            state.horizontal = (state.horizontal + 1).min(if state.vertical == row_count - 1 {
                let remainder = ((profile_count + HORIZONTAL_CHUNKS - 1) as f32)
                    / HORIZONTAL_CHUNKS as f32
                    % 1.;
                let mut min = HORIZONTAL_CHUNKS - 1;

                for i in 1..HORIZONTAL_CHUNKS {
                    if remainder < i as f32 / HORIZONTAL_CHUNKS as f32 {
                        min = i;
                        break;
                    }
                }

                min.min(profile_count - 1)
            } else {
                HORIZONTAL_CHUNKS - 1
            } as u8)
        }
        // select the profile
        KeyCode::Enter => {
            let profiles = chunk_profiles(app).collect::<Vec<_>>();

            if let Some(profile) = profiles[state.horizontal as usize][state.vertical] {
                app.selected = Some(profile.clone());
                change_view(app, Views::ViewProfile);
            }
        }
        _ => {}
    }
}
