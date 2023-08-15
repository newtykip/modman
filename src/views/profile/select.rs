use crate::prelude::*;
use ratatui::layout::Rect;

const CHUNK_WIDTH: u16 = 40;

// todo: make this scale to the terminal window size
fn chunk_profiles(app: &App, chunk_count: usize) -> impl Iterator<Item = Vec<Option<&Profile>>> {
    (0..chunk_count).map(move |i| {
        app.profiles
            .iter()
            .skip(i)
            .step_by(chunk_count)
            .map(Some)
            .collect()
    })
}

#[derive(Default)]
pub struct State {
    horizontal: u8,
    vertical: usize,
}

pub fn draw<B: Backend>(frame: &mut Frame<B>, app: &App, state: &mut State) {
    // divide it into chunks
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(app.profiles.len() as u16),
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
    // todo: preserve where you last were before resize?
    let chunk_count = ((frame.size().width / CHUNK_WIDTH) as usize).min(app.profiles.len());

    if state.horizontal > chunk_count as u8 - 1 {
        state.vertical = 0;
        state.horizontal = 0;
    }

    let list_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            (0..chunk_count)
                .map(|_| Constraint::Percentage(100 / chunk_count as u16))
                .collect::<Vec<_>>(),
        )
        .split(chunks[1]);

    let max_len = chunk_profiles(app, chunk_count)
        .map(|col| col.len())
        .max()
        .unwrap_or(0);

    for (i, mut profiles) in chunk_profiles(app, chunk_count).enumerate() {
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

        // footer
        let footer = Paragraph::new("You can create a new profile by pressing the c key.")
            .style(Style::default().fg(Color::DarkGray));

        frame.render_widget(footer, chunks[2]);
    }
}

pub fn controls(input: Input, app: &mut App, state: &mut State, frame_size: &Rect) {
    let chunk_count = ((frame_size.width / CHUNK_WIDTH) as usize).min(app.profiles.len());
    let profile_count = app.profiles.len();
    let row_count = (profile_count + chunk_count - 1) / chunk_count;

    match input.key {
        // move the cursor up
        Key::Up | Key::Char('w') => state.vertical = state.vertical.saturating_sub(1),
        // move the cursor down
        Key::Down | Key::Char('s') => {
            state.vertical = (state.vertical + 1).min(row_count - 1);

            // stop the horizontal index from going out of bounds
            state.horizontal = state.horizontal.min(if state.vertical == row_count - 1 {
                let remainder =
                    ((profile_count + chunk_count - 1) as f32) / chunk_count as f32 % 1.;
                let mut min = chunk_count - 1;

                for i in 1..chunk_count {
                    if remainder < i as f32 / chunk_count as f32 {
                        min = i;
                        break;
                    }
                }

                min
            } else {
                chunk_count - 1
            } as u8);
        }
        // move the cursor left
        Key::Left | Key::Char('a') => state.horizontal = state.horizontal.saturating_sub(1),
        // move the cursor right
        Key::Right | Key::Char('d') => {
            // stop the horizontal index from going out of bounds
            state.horizontal = (state.horizontal + 1).min(if state.vertical == row_count - 1 {
                let remainder =
                    ((profile_count + chunk_count - 1) as f32) / chunk_count as f32 % 1.;
                let mut min = chunk_count - 1;

                for i in 1..chunk_count {
                    if remainder < i as f32 / chunk_count as f32 {
                        min = i;
                        break;
                    }
                }

                min.min(profile_count - 1)
            } else {
                chunk_count - 1
            } as u8)
        }
        // select the profile
        Key::Enter => {
            let profiles = chunk_profiles(app, chunk_count).collect::<Vec<_>>();

            if let Some(profile) = profiles[state.horizontal as usize][state.vertical] {
                app.selected = Some(profile.clone());
                change_view(app, Views::ViewProfile);
            }
        }
        // swap to create profile
        Key::Char('c') => {
            change_view(app, Views::CreateProfile);
        }
        _ => {}
    }
}
