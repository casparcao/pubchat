pub mod normal_mode;
pub mod insert_mode;
pub mod send;

use crate::ui::models::{App, Mode};
use crossterm::event::KeyEvent;

impl App {
    pub fn handle_key_event(&mut self, key: KeyEvent) -> bool {
        match self.mode {
            Mode::Normal => self.handle_normal_mode_key_event(key),
            Mode::Insert => self.handle_insert_mode_key_event(key),
        }
    }
}