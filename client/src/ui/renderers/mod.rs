pub mod chat;
pub mod friends_list;
pub mod session;
pub mod login;


use crate::ui::models::{App, View};
use ratatui::{
    prelude::*,
};

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
            View::FriendsList => self.render_friends_list_layout(frame, size),
        }
    }
}