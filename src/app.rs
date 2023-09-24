#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub counter: i32,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn increment(&mut self) {
        self.counter += 1;
    }

    pub fn decrement(&mut self) {
        self.counter -= 1;
    }
    
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
