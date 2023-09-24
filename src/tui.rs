use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::io::{self, Stderr};

use crate::{app::App, event::EventHandler};

pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stderr>>,
    pub events: EventHandler,
}

impl Tui {
    // Constructor
    pub fn new(terminal: Terminal<CrosstermBackend<Stderr>>, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    // Initializes terminal, enables raw mode
    // Sets terminal properties
    pub fn init(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stderr(), EnterAlternateScreen)?;

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    // Exits the terminal interface
    pub fn exit(&mut self) -> io::Result<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stderr(), LeaveAlternateScreen)?;

        self.terminal.show_cursor()?;
        Ok(())
    }

    // Draws widgets on the terminal
    pub fn draw(&mut self, app: &mut App) -> io::Result<()> {
        self.terminal.draw(|frame| crate::ui::render(app, frame))?;
        Ok(())
    }
}
