pub mod chat;
pub mod friends_list;
pub mod session;


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
                    self.render_maximized_chat_layout(frame, size, target.clone())
                } else {
                    self.render_main_layout(frame, size, target.clone())
                }
            },
            View::FriendsList => self.render_friends_list_layout(frame, size),
        }
    }
}