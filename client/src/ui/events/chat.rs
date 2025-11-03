use crossterm::event::{KeyCode, KeyEvent};

use crate::ui::{models::Mode, screen::chat::ChatScreen};

impl ChatScreen {
    pub fn handle(&mut self, key: KeyEvent) -> bool {
        match self.chat.mode {
            Mode::Normal => {
                return self.handle_normal_mode(key);
            }
            Mode::Insert => {
                return self.handle_insert_mode(key);
            }
        }
    }

    pub fn handle_normal_mode(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('m') => {
                // 切换聊天窗口最大化状态
                self.maximized = !self.maximized;
            }
            KeyCode::Char('k') => {
                self.chat.scroll_up();
            }
            KeyCode::Char('j') => {
                self.chat.scroll_down();
            }
            _ => {}
        }
        false
    }

    pub fn handle_insert_mode(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Esc => {
                self.chat.change_mode(Mode::Normal);
            }
            KeyCode::Enter => {
                self.chat.send_message(stream);
            }
            KeyCode::Char(c) => {
                self.chat.input(c);
            }
            KeyCode::Backspace => {
                self.chat.delete();
            }
            _ => {}
        }
        false
    }
}