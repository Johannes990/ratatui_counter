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

// Some tests to our App struct functions
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_app_increment_counter() {
        let mut app = App::default();
        app.increment_counter();
        assert_eq!(app.counter, 1);
    }

    #[test]
    fn test_app_decrement_counter() {
        let mut app = App::default();
        app.decrement_counter();
        assert_eq!(app.counter, 0);
    }
}
