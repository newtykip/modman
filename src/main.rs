mod tui;

use std::io;

fn main() -> io::Result<()> {
    tui::prelude()?;

    Ok(())
}
