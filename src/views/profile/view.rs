use crate::prelude::*;

#[derive(Default)]
pub struct State {}

// todo: properly implement
pub fn draw<B: Backend>(frame: &mut Frame<B>, app: &App, _state: &State) {
    let profile = app.selected.as_ref().unwrap();

    // divide it into chunks
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2)])
        .split(frame.size());

    // render the header
    let header = Paragraph::new(vec![
        Line::from(vec![
            Span::styled(
                profile.name.clone(),
                Style::default().add_modifier(Modifier::UNDERLINED),
            ),
            Span::raw(format!(" v{} - ", profile.version)),
            Span::styled(
                format!(
                    "{} v{}",
                    profile.loader.to_string(),
                    profile.versions.get_version(Some(profile.loader))
                ),
                Style::default().add_modifier(Modifier::ITALIC),
            ),
        ]),
        Line::from(vec![Span::raw(format!("Author: {}", profile.author))]),
    ])
    .alignment(Alignment::Center);

    frame.render_widget(header, chunks[0]);
}

pub fn controls(input: Input, app: &mut App, _state: &mut State) {
    match input.key {
        Key::Backspace => {
            app.view = Views::SelectProfile;
        }
        _ => {}
    }
}
