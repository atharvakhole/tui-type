use std::{
    sync::mpsc::{self, RecvError},
    thread,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyEvent};

#[derive(Clone, Copy, Debug)]
pub enum Message {
    Tick,
    Input(KeyEvent),
}

#[derive(Debug)]
pub struct EventHandler {
    pub tx: mpsc::Sender<Message>,
    rx: mpsc::Receiver<Message>,
    _handle: thread::JoinHandle<()>,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (tx, rx) = mpsc::channel();
        let _handle = {
            let tx = tx.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    // Time left before next tick
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    if event::poll(timeout).expect("no events available") {
                        match event::read().expect("unable to read event") {
                            Event::Key(e) => {
                                if let Err(_) = tx.send(Message::Input(e)) {
                                    return;
                                }
                            }
                            _ => {}
                        }
                    }

                    if last_tick.elapsed() >= tick_rate {
                        tx.send(Message::Tick).expect("failed to send tick event");
                        last_tick = Instant::now();
                    }
                }
            })
        };
        Self { tx, rx, _handle }
    }

    pub fn next(&self) -> Result<Message, RecvError> {
        Ok(self.rx.recv()?)
    }
}
