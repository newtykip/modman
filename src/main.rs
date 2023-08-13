use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use modman::views::*;
use modman::App;
use ratatui::backend::CrosstermBackend;
use std::error::Error;
use std::io::{self, Stdout};
use std::time::Duration;
use std::time::Instant;

const TICK_RATE: Duration = Duration::from_millis(250);

struct Terminal(ratatui::Terminal<CrosstermBackend<Stdout>>);

impl Drop for Terminal {
    fn drop(&mut self) {
        disable_raw_mode()
            .and(execute!(self.0.backend_mut(), LeaveAlternateScreen))
            .and(self.0.show_cursor())
            .unwrap();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout();
    let mut terminal = Terminal(
        enable_raw_mode()
            .and(execute!(stdout, EnterAlternateScreen))
            .and(ratatui::Terminal::new(CrosstermBackend::new(stdout)))
            .unwrap(),
    );

    // states
    let mut app = App::default();
    let mut select_profile_state = profile::select::State::default();
    let mut view_profile_state = profile::view::State::default();

    let mut last_tick = Instant::now();

    loop {
        // draw view
        terminal.0.draw(|f| match app.view {
            Views::SelectProfile => profile::select::draw(f, &app, &select_profile_state),
            Views::ViewProfile => profile::view::draw(f, &app, &view_profile_state),

            #[allow(unreachable_patterns)]
            _ => unimplemented!(),
        })?;

        // delta time
        let timeout = TICK_RATE
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        // handle input
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                // default keybinds
                if let KeyCode::Esc = key.code {
                    return Ok(());
                }

                // view specific keybinds
                match app.view {
                    Views::SelectProfile => {
                        profile::select::controls(key.code, &mut app, &mut select_profile_state)
                    }
                    Views::ViewProfile => {
                        profile::view::controls(key.code, &mut app, &mut view_profile_state)
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
