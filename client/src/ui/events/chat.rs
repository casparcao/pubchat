use crossterm::event::{KeyCode, KeyEvent};

use crate::ui::{component::{chat::ChatComponent, session::SessionListComponent}, events::EventResult, models::Mode, screen::chat::{ChatScreen, Focus}};

impl ChatScreen {
    pub fn handle(&mut self, key: KeyEvent) -> EventResult{
        let result = match self.focus {
            Focus::Chat => {
                self.chat.handle(key)
            }
            Focus::Sessions => {
                self.sessions.handle(key)
            }
        };
        match result {
            EventResult::Nav2SessionList => {
                self.focus = Focus::Sessions;
            }
            _ => {}
        }
        result
    }


}

impl SessionListComponent {
    pub fn handle(&mut self, key: KeyEvent) -> EventResult {
        match key.code {
            KeyCode::Char('k') => {
                self.move_up();
            }
            KeyCode::Char('j') => {
                self.move_down();
            }
            KeyCode::Char('c') => {
                return EventResult::Nav2Contact;
            }
            KeyCode::Enter => {
                if let Some(session) = self.select() {
                    return EventResult::CreateSession(session.clone());
                }
            }
            _ => {}
        }
        EventResult::None
    }
}

impl ChatComponent {
    pub fn handle(&mut self, key: KeyEvent) -> EventResult  {
        match self.mode {
            Mode::Normal => {
                return self.handle_normal_mode(key);
            }
            Mode::Insert => {
                return self.handle_insert_mode(key);
            }
        }
    }

    pub fn handle_normal_mode(&mut self, key: KeyEvent) -> EventResult {
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
            KeyCode::Char('c') => {
                return EventResult::Nav2Contact;
            }
            KeyCode::Char('h') => {
                return EventResult::Nav2SessionList;
            }
            _ => {}
        }
        EventResult::None
    }

    pub fn handle_insert_mode(&mut self, key: KeyEvent) -> EventResult{
        match key.code {
            KeyCode::Esc => {
                self.change_mode(Mode::Normal);
            }
            KeyCode::Enter => {
                return EventResult::SendMessage();
            }
            KeyCode::Char(c) => {
                self.input(c);
            }
            KeyCode::Backspace => {
                self.delete();
            }
            _ => {}
        }
        EventResult::None
    }
}