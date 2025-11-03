use ratatui::Frame;

use crate::ui::{models::{App, View}, screen::{chat::ChatScreen, contact::ContactListScreen}};

pub mod login;
pub mod contact;
pub mod chat;

impl App {
    pub fn render(&self, frame: &mut Frame) {
        let size = frame.area();
        match &self.view {
            View::Chat { session } => {
                if self.chat_maximized {
                    self.chat.render_maximized_chat_layout(frame, size, session.clone())
                } else {
                    self.chat.render_main_layout(frame, size, session.clone())
                }
            },
            View::Contact => {
                self.contact.render_friends_list_layout(frame, size)
            },
        }
    }
}