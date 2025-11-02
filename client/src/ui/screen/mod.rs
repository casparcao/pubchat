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
                let chat_screen = ChatScreen::new(&self.token);
                if self.chat_maximized {
                    chat_screen.render_maximized_chat_layout(frame, size, session.clone())
                } else {
                    chat_screen.render_main_layout(frame, size, session.clone())
                }
            },
            View::Contact => {
                let contact_list = ContactListScreen::new(&self.token);
                contact_list.render_friends_list_layout(frame, size)
            },
        }
    }
}