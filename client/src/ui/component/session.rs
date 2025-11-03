use core::request::Page;

use ratatui::{Frame, layout::Rect, widgets::{Block, Borders, List, ListItem}};

use crate::{cache, ui::models::Session};

#[derive(Debug, Clone)]
pub struct SessionListComponent {
    pub sessions: Vec<Session>,
}

impl SessionListComponent {
    pub fn new(token: &str) -> Self {
        match cache::session_cache().get_sessions(token, Page::default()){
            Ok(sessions) => {
                Self {sessions: sessions
                    .into_iter()
                    .map(|s| crate::ui::models::Session::from_session_response(s))
                    .collect()}
            },
            Err(e) => {
                log::error!("Failed to get sessions: {:?}", e);
                Self {sessions: vec![]}
            },
        }
    }
    
    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let sessions: Vec<ListItem> = self.sessions
            .iter()
            .enumerate()
            .map(|(i, session)| {
                let content = format!("💬 {}", session.name);
                let item = ListItem::new(content);
                item
            })
            .collect();
        let title = "Sessions";
        let sessions_list = List::new(sessions)
            .block(Block::default().title(title).borders(Borders::ALL));

        frame.render_widget(sessions_list, area);
    }
}
