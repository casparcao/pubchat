use crate::ui::models::{App, View, Mode, Status};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

impl App {
    pub fn render_maximized_chat_layout(&self, frame: &mut Frame, area: Rect, target: &str) {
        // 最大化聊天窗口布局：只显示聊天窗口和输入框
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),         // 消息区域（占据大部分空间）
                Constraint::Length(5),      // 输入框区域
            ])
            .split(area);

        self.render_messages(frame, chunks[0]);
        self.render_input(frame, chunks[1]);
    }

    pub fn render_main_layout(&self, frame: &mut Frame, area: Rect, target: &str) {
        // 两栏布局：会话列表(1/3) + 聊天窗口(2/3)
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30), // 会话列表
                Constraint::Percentage(70), // 聊天窗口
            ])
            .split(area);

        // 左侧会话列表
        self.render_sessions_list(frame, chunks[0]);

        // 右侧聊天区域
        let chat_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),         // 消息区域
                Constraint::Length(5),      // 增大输入框区域
            ])
            .split(chunks[1]);

        self.render_messages(frame, chat_chunks[0]);
        self.render_input(frame, chat_chunks[1]);
    }

    pub fn render_messages(&self, frame: &mut Frame, area: Rect) {
        // 获取当前聊天目标的消息
        let messages = match &self.current_view {
            View::Chat { target } => {
                self.messages.get(target).cloned().unwrap_or_default()
            },
            _ => vec![]
        };

        let list_items: Vec<ListItem> = messages.iter().map(|m| {
            let style = if m.is_user {
                Style::default().fg(Color::Blue)
            } else if m.sender == "SYSTEM" {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::Green)
            };

            let content = format!("[{}] <{}> {}", m.timestamp, m.sender, m.content);
            ListItem::new(content).style(style)
        }).collect();

        // 获取当前聊天目标
        let title = match &self.current_view {
            View::Chat { target } => {
                // 检查目标是会话还是群组
                if self.sessions.iter().any(|s| s.name == *target) {
                    format!("Chat with {} {}", target, 
                        if self.chat_maximized { "[M] (Press 'm' to restore)" } else { "[M] (Press 'm' to maximize)" })
                } else {
                    format!("Chat with {} {}",
                        target,
                        if self.chat_maximized { "[M] (Press 'm' to restore)" } else { "[M] (Press 'm' to maximize)" })
                }
            },
            _ => "Messages".to_string(),
        };

        let messages_list = List::new(list_items)
            .block(Block::default().title(title).borders(Borders::ALL))
            .scroll_padding(1);

        frame.render_widget(messages_list, area);
    }

    pub fn render_input(&self, frame: &mut Frame, area: Rect) {
        let (text, style) = match self.mode {
            Mode::Normal => ("Normal Mode (i to insert)", Style::default().fg(Color::Yellow)),
            Mode::Insert => ("INSERT (Esc to normal)", Style::default().fg(Color::Green)),
        };

        // 创建一个内部区域，保留底部一行用于模式提示
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),        // 输入区域
                Constraint::Length(1),     // 模式提示
            ])
            .split(area);

        let input = Paragraph::new(self.input.as_str())
            .block(Block::default().borders(Borders::ALL));

        let mode = Paragraph::new(text)
            .style(style)
            .alignment(Alignment::Left);

        frame.render_widget(input, chunks[0]);
        frame.render_widget(mode, chunks[1]);
        
        // 只在插入模式下设置光标位置
        if let Mode::Insert = self.mode {
            frame.set_cursor_position(
                (chunks[0].x + self.input.len() as u16 + 1,
                 chunks[0].y + 1)
            );
        }
    }
}