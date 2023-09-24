use std::io;

use ratatui::{prelude::CrosstermBackend, Terminal};
use tui_type::{
    app::App,
    event::{EventHandler, Message},
    handler,
    tui::Tui,
};

fn main() -> io::Result<()> {
    // Initialize app
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;

    // We will later accept terminal arguments for tick rate
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    while !app.should_quit {
        // Render
        tui.draw(&mut app)?;

        // We are using `unwrap` as we haven't implemented dynamic dispatch
        // and wrapped our errors yet
        let next_event = tui.events.next().unwrap();
        match next_event {
            Message::Input(_) => {
                let mut current_message = handler::reduce(&mut app, next_event);

                // Reduce events until the reducer returns no further messages
                while current_message.is_some() {
                    current_message = handler::reduce(&mut app, current_message.unwrap())
                }
            }
            Message::Tick => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
