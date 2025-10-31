pub mod chat;
pub mod contact;
pub mod session;
pub mod login;


use crate::ui::models::{App, View};
use ratatui::{
    prelude::*,
};
use contact::ContactListComponent;

impl App {
    pub fn render(&self, frame: &mut Frame) {
        let size = frame.area();
        match &self.current_view {
            View::Chat { session } => {
                if self.chat_maximized {
                    self.render_maximized_chat_layout(frame, size, session.clone())
                } else {
                    self.render_main_layout(frame, size, session.clone())
                }
            },
            View::FriendsList => {
                let contact_list = ContactListComponent::new(&self.token);
                contact_list.render_friends_list_layout(frame, size)
            },
        }
    }
}