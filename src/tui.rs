use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::{
    prelude::{Alignment, Backend},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Borders, Paragraph},
    Terminal,
};
use std::io;

pub struct Tui<B: Backend> {
    terminal: Terminal<B>,
}

impl<B: Backend> Tui<B> {
    // Constructor
    pub fn new(terminal: Terminal<B>) -> Self {
        Self { terminal }
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
    pub fn draw(&mut self) -> io::Result<()> {
        self.terminal.draw(|frame| {
            frame.render_widget(
                Paragraph::new(format!("Hello world"))
                    .block(
                        Block::default()
                            .title("Template")
                            .title_alignment(Alignment::Center)
                            .border_style(Style::default().fg(Color::White))
                            .borders(Borders::ALL)
                            .border_type(BorderType::Thick),
                    )
                    .style(Style::default().fg(Color::Cyan).bg(Color::Black).bold())
                    .alignment(Alignment::Left),
                frame.size(),
            );
        })?;
        Ok(())
    }
}
