use crate::ui::models::{App, View, Me};
use crate::ui::screen::contact::ContactListScreen;
use crate::ui::screen::chat::ChatScreen;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::tcp::OwnedWriteHalf;

impl App {
    pub fn new(token: String, me: Me, stream: Arc<Mutex<OwnedWriteHalf>>) -> Self {
        Self {
            contact: ContactListScreen::new(&token),
            chat: ChatScreen::new(&token, me, stream),
            view: View::Contact,
            token: token,
        }
    }
    
}
