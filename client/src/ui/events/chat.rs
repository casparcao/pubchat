use crossterm::event::{KeyCode, KeyEvent};

use crate::ui::{component::{chat::ChatComponent, session::SessionListComponent}, models::Mode, screen::chat::{ChatScreen, Focus}};

impl ChatScreen {
    pub fn handle(&mut self, key: KeyEvent) {
        match self.focus {
            Focus::Chat => {
                self.chat.handle(key);
            }
            Focus::Sessions => {
                self.sessions.handle(key);
            }
        }
        match key.code {
            KeyCode::Char('h') => {
                self.focus_left();
            }
            KeyCode::Char('l') => {
                self.focus_right();
            }
        }
    }

    pub fn handle_normal_mode(&mut self, key: KeyEvent) {
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
    }

    pub fn handle_insert_mode(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.chat.change_mode(Mode::Normal);
            }
            KeyCode::Enter => {
                self.chat.send_message(&self.me, &self.stream);
            }
            KeyCode::Char(c) => {
                self.chat.input(c);
            }
            KeyCode::Backspace => {
                self.chat.delete();
            }
            _ => {}
        }
    }
}

impl SessionListComponent {
    pub fn handle(&mut self, key: KeyEvent)  {
        match key.code {
            KeyCode::Char('k') => {
                self.move_up();
            }
            KeyCode::Char('j') => {
                self.move_down();
            }
            KeyCode::Enter => {
                if let Some(session) = self.get_selected_session() {
                    self.change_session(session);
                }
            }
        }
    }
}

impl ChatComponent {
    pub fn handle(&mut self, key: KeyEvent)  {
        match key.code {
            KeyCode::Char('i') => {
                self.change_mode(Mode::Insert);
            }
            KeyCode::Char('k') => {
                self.scroll_up();
            }
            KeyCode::Char('j') => {
                self.scroll_down();
            }
            KeyCode::Char('h') => {
}
        }
    }
}