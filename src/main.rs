use std::{io, thread, time::Duration};

use ratatui::{prelude::CrosstermBackend, Terminal};
use tui_type::tui::Tui;

fn main() -> io::Result<()> {
    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let mut tui = Tui::new(terminal);
    tui.init()?;

    tui.draw()?;
    thread::sleep(Duration::from_millis(5000));

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
