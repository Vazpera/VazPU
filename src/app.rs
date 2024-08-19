use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;
#[derive(Debug)]
pub enum CurrentScreen {
    Counter1,
    Counter2,
}
/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub curr: CurrentScreen,
    pub running: bool,
    /// counters
    pub counter1: u8,
    pub counter2: u8,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter1: 0,
            counter2: 0,
            curr: CurrentScreen::Counter1,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        match self.curr {
            CurrentScreen::Counter1 => {
                if let Some(res) = self.counter1.checked_add(1) {
                    self.counter1 = res;
                }
            }
            CurrentScreen::Counter2 => {
                if let Some(res) = self.counter2.checked_add(1) {
                    self.counter2 = res;
                }
            }
        }
    }

    pub fn decrement_counter(&mut self) {
        match self.curr {
            CurrentScreen::Counter1 => {
                if let Some(res) = self.counter1.checked_sub(1) {
                    self.counter1 = res;
                }
            }
            CurrentScreen::Counter2 => {
                if let Some(res) = self.counter2.checked_sub(1) {
                    self.counter2 = res;
                }
            }
        }
    }

    pub fn change_screen(&mut self) {
        self.curr = match self.curr {
            CurrentScreen::Counter1 => CurrentScreen::Counter2,
            CurrentScreen::Counter2 => CurrentScreen::Counter1,
        }
    }
}
