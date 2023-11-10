#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub counter: u8,
}

impl App {
    pub fn new() -> Self {
        // construct a new instance of 'App'
        Self::default()
    }

    // handle the tick event of the terminal
    pub fn tick(&self) {}

    // set running to false to quit the application
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}