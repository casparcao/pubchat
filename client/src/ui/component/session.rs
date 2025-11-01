use ratatui::{Frame, layout::Rect, widgets::{Block, Borders, List, ListItem}};

use crate::ui::models::Session;

#[derive(Debug, Clone)]
pub struct SessionListComponent {
    pub sessions: Vec<Session>,
}

impl SessionListComponent {
    pub fn new(sessions: Vec<Session>) -> Self {
        Self {
            sessions
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
