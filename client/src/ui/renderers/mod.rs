pub mod chat;
pub mod contacts;
pub mod friends_list;


use crate::ui::models::{App, View};
use ratatui::{
    prelude::*,
};

impl App {
    pub fn render(&self, frame: &mut Frame) {
        let size = frame.area();
        match &self.current_view {
            View::Chat { target } => {
                if self.chat_maximized {
                    self.render_maximized_chat_layout(frame, size, target)
                } else {
                    self.render_main_layout(frame, size, target)
                }
            },
            View::Contacts => self.render_contacts_layout(frame, size),
            View::FriendsList => self.render_friends_list_layout(frame, size),
        }
    }
}