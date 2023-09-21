use std::{io, thread, time::Duration};

use ratatui::{prelude::CrosstermBackend, Terminal};
use tui_type::{tui::Tui, event::EventHandler};

fn main() -> io::Result<()> {
    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;

    // We will later accept terminal arguments for tick rate
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    tui.draw()?;
    thread::sleep(Duration::from_millis(5000));

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
