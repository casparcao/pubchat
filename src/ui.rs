use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

// 应用状态
#[derive(Debug, Clone)]
pub struct App {
    pub input: String,
    pub messages: Vec<Message>,
    pub contacts: Vec<String>,
    pub groups: Vec<String>,
    pub current_view: View,
    pub mode: Mode,
    pub scroll_offset: u16,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub sender: String,
    pub content: String,
    pub timestamp: String,
    pub is_user: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum View {
    Chat { target: String },
    Contacts,
    Groups,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Normal,
    Insert,
}

impl Default for App {
    fn default() -> Self {
        Self {
            input: String::new(),
            messages: vec![
                Message::system("Welcome to Chat Terminal! Type /help for commands."),
                Message::system("Connected as 'user1'."),
            ],
            contacts: vec!["alice".to_string(), "bob".to_string()],
            groups: vec!["dev-team".to_string(), "random".to_string()],
            current_view: View::Chat {
                target: "alice".to_string(),
            },
            mode: Mode::Normal,
            scroll_offset: 0,
        }
    }
}

impl Message {
    pub fn new(sender: String, content: String, is_user: bool) -> Self {
        Self {
            sender,
            content,
            timestamp: chrono::Local::now().format("%H:%M").to_string(),
            is_user,
        }
    }

    pub fn system(content: &str) -> Self {
        Self::new("SYSTEM".to_string(), content.to_string(), false)
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) {
        match self.mode {
            Mode::Normal => match key.code {
                crossterm::event::KeyCode::Char('q') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => {
                    // Ctrl+Q 退出
                    std::process::exit(0);
                }
                crossterm::event::KeyCode::Char('i') => {
                    self.mode = Mode::Insert;
                }
                crossterm::event::KeyCode::Char('k') => {
                    if self.scroll_offset > 0 {
                        self.scroll_offset -= 1;
                    }
                }
                crossterm::event::KeyCode::Char('j') => {
                    self.scroll_offset += 1;
                }
                crossterm::event::KeyCode::Char('h') => {
                    // 切换到联系人视图
                    self.current_view = View::Contacts;
                }
                crossterm::event::KeyCode::Char('g') => {
                    // 切换到群组视图
                    self.current_view = View::Groups;
                }
                _ => {}
            },
            Mode::Insert => match key.code {
                crossterm::event::KeyCode::Esc => {
                    self.mode = Mode::Normal;
                }
                crossterm::event::KeyCode::Enter => {
                    self.send_message();
                }
                crossterm::event::KeyCode::Char(c) => {
                    self.input.push(c);
                }
                crossterm::event::KeyCode::Backspace => {
                    self.input.pop();
                }
                _ => {}
            },
        }
    }

    fn send_message(&mut self) {
        if self.input.is_empty() {
            return;
        }

        // 处理命令
        if self.input.starts_with('/') {
            self.handle_command();
        } else {
            // 发送普通消息
            let sender = match &self.current_view {
                View::Chat { target } => target.clone(),
                _ => "unknown".to_string(),
            };
            let msg = Message::new("You".to_string(), self.input.clone(), true);
            self.messages.push(msg);
            // TODO: 实际发送到网络
            // self.network.send(MessagePacket { ... });
        }

        self.input.clear();
        self.mode = Mode::Normal;
    }

    fn handle_command(&mut self) {
        let cmd = self.input.split_whitespace().next().unwrap_or("");
        match cmd {
            "/help" => {
                self.messages.push(Message::system("Commands: /help, /clear, /quit, /list"));
            }
            "/clear" => {
                self.messages.clear();
            }
            "/quit" => {
                std::process::exit(0);
            }
            "/list" => {
                let contact_list = self.contacts.join(", ");
                let group_list = self.groups.join(", ");
                self.messages.push(Message::system(&format!("Contacts: {}", contact_list)));
                self.messages.push(Message::system(&format!("Groups: {}", group_list)));
            }
            _ => {
                self.messages.push(Message::system(&format!("Unknown command: {}", cmd)));
            }
        }
        self.input.clear();
        self.mode = Mode::Normal;
    }

    pub fn render(&self, frame: &mut Frame) {
        let size = frame.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),
                Constraint::Length(3), // 输入框高度
            ])
            .split(size);

        // 主聊天窗口
        self.render_messages(frame, chunks[0]);

        // 输入框
        self.render_input(frame, chunks[1]);
    }

    fn render_messages(&self, frame: &mut Frame, area: Rect) {
        let messages: Vec<ListItem> = self.messages.iter().map(|m| {
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

        let messages_list = List::new(messages)
            .block(Block::default().title("Messages").borders(Borders::ALL))
            .scroll_padding(1);

        frame.render_widget(messages_list, area);
        frame.set_cursor(
            area.x + self.input.len() as u16 + 1,
            area.y + 1,
        );
    }

    fn render_input(&self, frame: &mut Frame, area: Rect) {
        let (text, style) = match self.mode {
            Mode::Normal => ("Normal Mode", Style::default().fg(Color::Yellow)),
            Mode::Insert => ("INSERT", Style::default().fg(Color::Green)),
        };

        let mode = Paragraph::new(text)
            .style(style)
            .alignment(Alignment::Right);

        let input = Paragraph::new(self.input.as_str())
            .block(Block::default().title("Enter message").borders(Borders::ALL));

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(10), Constraint::Length(10)])
            .split(area);

        frame.render_widget(input, chunks[0]);
        frame.render_widget(mode, chunks[1]);
    }
}