use crossterm::event::{KeyCode, KeyEvent};

use crate::ui::{events::EventResult, screen::contact::ContactListScreen};

impl ContactListScreen {
    pub fn handle(&mut self, key: KeyEvent) -> EventResult  {
        match key.code {
            KeyCode::Char('k') => {
                self.move_up();
            }
            KeyCode::Char('j') => {
                self.move_down();
            }
            KeyCode::Enter | KeyCode::Char('l') => {
                if let Ok(session ) = self.create_session(){
                    return EventResult::CreateSession(session);
                }
            }
            _ => {}
        }
        EventResult::None
    }
}