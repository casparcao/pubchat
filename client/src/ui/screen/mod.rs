use ratatui::Frame;

use crate::ui::{models::{App, View}};

pub mod login;
pub mod contact;
pub mod chat;

impl App {
    pub fn render(&self, frame: &mut Frame) {
        let size = frame.area();
        match &self.view {
            View::Chat => {
                self.chat.render(frame, size)
            },
            View::Contact => {
                self.contact.render_friends_list_layout(frame, size)
            },
        }
    }

    pub fn change_view(&mut self, view: View) {
        self.view = view;
    }
}