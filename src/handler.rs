use crossterm::event::{KeyCode, KeyModifiers};

use crate::{app::App, event::Message};

pub fn reduce(app: &mut App, event: Message) -> Option<Message> {
    match event {
        Message::Input(key_event) => match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                app.quit();
                None
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.quit();
                }
                None
            }
            KeyCode::Right | KeyCode::Char('j') => {
                app.increment();
                None
            }
            KeyCode::Left | KeyCode::Char('k') => {
                app.decrement();
                None
            }
            _ => None,
        },
        _ => None,
    }
}
