use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;
#[derive(Debug)]
pub enum Current_Screen {
    Counter_1,
    Counter_2,
}
/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub curr: Current_Screen,
    pub running: bool,
    /// counters
    pub counter_1: u8,
    pub counter_2: u8,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter_1: 0,
            counter_2: 0,
            curr: Current_Screen::Counter_1,
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
            Current_Screen::Counter_1 => {
                if let Some(res) = self.counter_1.checked_add(1) {
                    self.counter_1 = res;
                }
            }
            Current_Screen::Counter_2 => {
                if let Some(res) = self.counter_2.checked_add(1) {
                    self.counter_2 = res;
                }
            }
        }
    }

    pub fn decrement_counter(&mut self) {
        match self.curr {
            Current_Screen::Counter_1 => {
                if let Some(res) = self.counter_1.checked_sub(1) {
                    self.counter_1 = res;
                }
            }
            Current_Screen::Counter_2 => {
                if let Some(res) = self.counter_2.checked_sub(1) {
                    self.counter_2 = res;
                }
            }
        }
    }

    pub fn change_screen(&mut self) {
        self.curr = match self.curr {
            Current_Screen::Counter_1 => Current_Screen::Counter_2,
            Current_Screen::Counter_2 => Current_Screen::Counter_1,
        }
    }
}
