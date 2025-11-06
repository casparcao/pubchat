use core::request::Page;

use ratatui::{Frame, layout::Rect, style, widgets::{Block, Borders, List, ListItem}};

use crate::{cache, ui::{models::Session, screen::chat::Focus}};

#[derive(Debug, Clone)]
pub struct SessionListComponent {
    pub sessions: Vec<Session>,
    // 当前选中的会话索引
    pub index: usize,
}

impl SessionListComponent {
    pub fn new(token: &str) -> Self {
        match cache::session_cache().get_sessions(token, Page::default()){
            Ok(sessions) => {
                Self {sessions: sessions
                    .into_iter()
                    .map(|s| crate::ui::models::Session::from_session_response(s))
                    .collect(), index: 0}
            },
            Err(e) => {
                log::error!("Failed to get sessions: {:?}", e);
                Self {sessions: vec![], index: 0}
            },
        }
    }
    
    pub fn render(&self, frame: &mut Frame, area: Rect, focus: &Focus) {
        let sessions: Vec<ListItem> = self.sessions
            .iter()
            .enumerate()
            .map(|(i, session)| {
                if i == self.index {
                    ListItem::new(format!("{}", session.name))
                        .style(style::Style::default().bg(style::Color::Blue).fg(style::Color::White))
                } else {
                    ListItem::new(format!("{}", session.name))
                }
            })
            .collect();
        let title = "Sessions";
        let style = match focus {
            Focus::Sessions => style::Style::default().fg(style::Color::Yellow),
            _ => style::Style::default(),
        };
        let sessions_list = List::new(sessions)
            .block(Block::default().title(title).style(style).borders(Borders::ALL));
        frame.render_widget(sessions_list, area);
    }

    pub fn move_up(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.index + 1 < self.sessions.len() {
            self.index += 1;
        }
    }

    pub fn select(&self) -> Option<&Session> {
        self.sessions.get(self.index)
    }

    pub fn add_session(&mut self, session: Option<Session>) {
        if let Some(session) = session {
            if !self.sessions.iter().any(|s| s.id == session.id) {
                self.sessions.push(session);
            }
        }
    }
}
