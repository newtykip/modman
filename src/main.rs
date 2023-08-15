use backtrace::Backtrace;
use color_eyre::eyre::Result;
use crossterm::event::{self, Event};
use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use modman::views::*;
use modman::App;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::Rect;
use std::io::{self};
use std::panic::{self, PanicInfo};
use std::time::Duration;
use std::time::Instant;
use tui_textarea::{Input, Key};

const TICK_RATE: Duration = Duration::from_millis(250);

fn close_application() -> Result<()> {
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}

fn panic_hook(info: &PanicInfo<'_>) {
    if cfg!(debug_assertions) {
        let location = info.location().unwrap();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>",
            },
        };

        let stacktrace: String = format!("{:?}", Backtrace::new()).replace('\n', "\n\r");

        disable_raw_mode().unwrap();
        execute!(
            io::stdout(),
            LeaveAlternateScreen,
            Print(format!(
                "thread '<unnamed>' panicked at '{}', {}\n\r{}",
                msg, location, stacktrace
            )),
        )
        .unwrap();
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    panic::set_hook(Box::new(panic_hook));

    let mut stdout = io::stdout();
    let mut terminal = enable_raw_mode()
        .and(execute!(stdout, EnterAlternateScreen))
        .and(ratatui::Terminal::new(CrosstermBackend::new(stdout)))
        .unwrap();

    // states
    let mut app = App::default();
    let mut select_profile_state = profile::select::State::default();
    let mut view_profile_state = profile::view::State::default();
    let mut create_profile_state = profile::create::State::default();

    let mut last_tick = Instant::now();
    let mut frame_size = Rect::default();

    loop {
        // draw view
        terminal.draw(|frame| {
            frame_size = frame.size();

            match app.view {
                Views::SelectProfile => {
                    profile::select::draw(frame, &app, &mut select_profile_state)
                }
                Views::ViewProfile => profile::view::draw(frame, &app, &view_profile_state),
                Views::CreateProfile => profile::create::draw(frame, &app, &create_profile_state),
            }
        })?;

        // delta time
        let timeout = TICK_RATE
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        // handle input
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                let input: Input = key.into();

                // default keybinds
                if let Key::Esc = input.key {
                    break;
                }

                // view specific keybinds
                match app.view {
                    Views::SelectProfile => profile::select::controls(
                        input,
                        &mut app,
                        &mut select_profile_state,
                        &frame_size,
                    ),
                    Views::ViewProfile => {
                        profile::view::controls(input, &mut app, &mut view_profile_state)
                    }
                    Views::CreateProfile => {
                        profile::create::controls(input, &mut app, &mut create_profile_state)
                    }
                }
            }
        }

        if last_tick.elapsed() >= TICK_RATE {
            last_tick = Instant::now();
        }
    }

    close_application()
}
