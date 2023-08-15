use super::{Stage, State};
use crate::prelude::*;
use crate::structs::Loader;
use crate::{input_keybinds, App};
use ratatui::layout::Rect;
use semver::Version;
use std::collections::HashMap;
use strum::{EnumIter, IntoEnumIterator};
use tui_textarea::{Input, Key, TextArea};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, EnumIter)]
enum InputField {
    #[default]
    Name,
    Author,
    Version,
}

impl Into<Line<'_>> for InputField {
    fn into(self) -> Line<'static> {
        Line::from(match self {
            InputField::Name => "Name: ",
            InputField::Author => "Author: ",
            InputField::Version => "Modpack Version: ",
        })
    }
}

impl InputField {
    fn last(&self) -> Self {
        match self {
            InputField::Name => InputField::Version,
            InputField::Author => InputField::Name,
            InputField::Version => InputField::Author,
        }
    }

    fn next(&self) -> Self {
        match self {
            InputField::Name => InputField::Author,
            InputField::Author => InputField::Version,
            InputField::Version => InputField::Name,
        }
    }

    fn validate(&self, value: impl Into<String>) -> bool {
        let value: String = value.into();

        match self {
            InputField::Name | InputField::Author => {
                let max_len = if let InputField::Name = self { 20 } else { 30 };

                !(value.is_empty()
                    || !value.chars().all(|c| c.is_alphanumeric())
                    || value.len() > max_len)
            }
            InputField::Version => Version::parse(&value).is_ok(),
        }
    }
}

#[derive(Clone)]
pub struct InitialState {
    selected: InputField,
    valid: HashMap<InputField, bool>,
    fields: HashMap<InputField, TextArea<'static>>,
    loader: Loader,
}

impl Default for InitialState {
    fn default() -> Self {
        let mut area = TextArea::default();
        area.set_cursor_style(Style::default());

        Self {
            selected: InputField::default(),
            valid: InputField::iter()
                .map(|variant| (variant, true))
                .collect::<HashMap<_, _>>(),
            fields: InputField::iter()
                .map(|variant| (variant, area.clone()))
                .collect::<HashMap<_, _>>(),
            loader: Loader::default(),
        }
    }
}

impl InitialState {
    fn draw_textarea<'a>(&self, input: InputField, rect: Rect, frame: &mut Frame<impl Backend>) {
        let label: Line = input.clone().into();
        let text_area = self.fields.get(&input).unwrap();
        let valid = self.valid.get(&input).unwrap();

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(label.width() as u16), Constraint::Min(1)])
            .split(rect);

        frame.render_widget(
            Paragraph::new(vec![label]).style({
                let mut style = Style::default();

                if self.selected != input {
                    style = style.fg(Color::DarkGray);
                }

                if !valid {
                    style = style.fg(Color::Red);
                }

                style
            }),
            chunks[0],
        );

        frame.render_widget(text_area.widget(), chunks[1]);
    }
}

pub fn draw<B: Backend>(frame: &mut Frame<B>, _app: &App, state: &InitialState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(frame.size());

    // draw header
    let header = Paragraph::new(vec![
        Line::from(Span::styled(
            "Create a new profile",
            Style::default().fg(Color::Green),
        )),
        Line::from(Span::styled(
            "First provide some basic details",
            Style::default().fg(Color::DarkGray),
        )),
    ])
    .alignment(Alignment::Center);

    frame.render_widget(header, chunks[0]);

    // draw inputs
    state.draw_textarea(InputField::Name, chunks[1], frame);
    state.draw_textarea(InputField::Author, chunks[2], frame);
    state.draw_textarea(InputField::Version, chunks[3], frame);
}

pub fn controls(input: Input, _app: &mut App, super_state: &mut State) {
    let state = &mut super_state.initial;
    let selected_area = state.fields.get_mut(&state.selected).unwrap();
    input_keybinds(selected_area, &input);

    if let Key::Enter = input.key {
        // check if fields are valid
        let mut results = vec![];

        for variant in InputField::iter() {
            let area = state.fields.get_mut(&variant).unwrap();
            let valid = state.valid.get_mut(&variant).unwrap();

            let res = variant.validate(area.lines().join(""));
            *valid = res;
            results.push(res);
        }

        // if all fields are valid, move onto the next stage
        if results.iter().all(|x| *x) && state.loader != Loader::Unknown {
            super_state.stage = Stage::MinecraftVersion;
        }
    } else {
        match input.key {
            Key::Up => {
                state.selected = state.selected.last();
            }
            Key::Down => {
                state.selected = state.selected.next();
            }
            _ => {}
        }

        if state.selected == InputField::Version {
            if let Key::Char('1')
            | Key::Char('2')
            | Key::Char('3')
            | Key::Char('4')
            | Key::Char('5')
            | Key::Char('6')
            | Key::Char('7')
            | Key::Char('8')
            | Key::Char('9')
            | Key::Char('0')
            | Key::Char('.')
            | Key::Backspace = input.key
            {
                selected_area.input(input);
            }
        } else {
            selected_area.input(input);
        }

        // reset all validation checks
        state.valid = InputField::iter()
            .map(|variant| (variant, true))
            .collect::<HashMap<_, _>>();
    }
}
