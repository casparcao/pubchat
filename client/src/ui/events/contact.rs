use crossterm::event::{KeyCode, KeyEvent};

use crate::ui::screen::contact::ContactListScreen;

impl ContactListScreen {
    pub fn handle(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('i') => {
            }
            KeyCode::Char('k') => {
                self.move_up();
            }
            KeyCode::Char('j') => {
                self.move_down();
            }
            KeyCode::Char('f') => {
            }
            KeyCode::Char('m') => {
            }
            KeyCode::Enter => {
                if let Ok(session ) = self.create_session(&self.token, &self.me){
                    self.view = View::Chat;
                    self.chat.chat.change_session(session);
                }
            }
            _ => {}
        }
        false
    }
}