pub mod send;
pub mod chat;
pub mod contact;

use crate::ui::models::{App, View};
use crossterm::event::{KeyCode, KeyEvent};

impl App {
    pub fn handle(&mut self, key: KeyEvent) -> bool {
        match self.view {
            View::Contact => self.contact.handle(key),
            View::Chat => self.chat.handle(key),
        };
        match key.code {
            KeyCode::Tab => {
                // 在不同视图间切换
                self.view = match self.view {
                    View::Chat => View::Contact,
                    View::Contact => View::Chat,
                };
            }
        }
        false
    }
}