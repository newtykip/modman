mod helpers;
mod views;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use modman::{load_profiles, structs::Profile};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};
use views::profile;

const TICK_RATE: Duration = Duration::from_millis(250);

#[derive(Clone, Copy)]
pub enum View {
    /// select profile screen
    SelectProfile,
    /// bool represents whether this is the first time creating a profile
    CreateProfile(bool), // todo: implement
    /// view profile screen
    ViewProfile,
}

pub struct App {
    profiles: Vec<Profile>,
    profile: Option<Profile>,
    view: View,
    previous_view: Option<View>,
}

impl App {
    pub fn new() -> io::Result<Self> {
        let profiles = load_profiles()?;

        Ok(Self {
            profiles: profiles.clone(),
            profile: None,
            view: if profiles.is_empty() {
                View::CreateProfile(true)
            } else {
                View::SelectProfile
            },
            previous_view: None,
        })
    }
}

pub fn prelude() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_tui(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}

pub fn run_tui<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut last_tick = Instant::now();

    // states
    let mut app = App::new()?;
    let mut select_profile_state = profile::select::State::default();
    let mut view_profile_state = profile::view::State::default();

    loop {
        // draw ui
        terminal.draw(|f| match app.view {
            View::SelectProfile => profile::select::draw(f, &app, &select_profile_state),

            View::ViewProfile => profile::view::draw(f, &app, &view_profile_state),
            #[allow(unreachable_patterns)]
            _ => unimplemented!(),
        })?;

        // delta time
        let timeout = TICK_RATE
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                // default keybinds
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    _ => {}
                }

                // view specific keybinds
                match app.view {
                    View::SelectProfile => {
                        profile::select::controls(&key.code, &mut app, &mut select_profile_state)
                    }

                    View::ViewProfile => {
                        profile::view::controls(&key.code, &mut app, &mut view_profile_state)
                    }
                    #[allow(unreachable_patterns)]
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= TICK_RATE {
            last_tick = Instant::now();
        }
    }
}
