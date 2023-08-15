pub use crate::{change_view, structs::Profile, views::Views, App};
pub use color_eyre::eyre::Result;
pub use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Text,
    text::{Line, Span},
    widgets::{List, ListItem, ListState, Paragraph},
    Frame,
};
pub use tui_textarea::{Input, Key};
