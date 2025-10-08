use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::collections::HashMap;

// 应用状态
#[derive(Debug, Clone)]
pub struct App {
    pub input: String,
    pub messages: HashMap<String, Vec<Message>>,
    pub contacts: Vec<Contact>,
    pub groups: Vec<Group>,
    pub current_view: View,
    pub mode: Mode,
    pub scroll_offset: u16,
    pub selected_contact: Option<usize>,
    pub selected_group: Option<usize>,
    pub current_user: String,
    pub chat_maximized: bool, // 添加最大化状态字段
    // 添加token字段存储用户认证信息
    pub token: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub sender: String,
    pub content: String,
    pub timestamp: String,
    pub is_user: bool,
}

#[derive(Debug, Clone)]
pub struct Contact {
    pub name: String,
    pub status: Status,
}

#[derive(Debug, Clone)]
pub struct Group {
    pub name: String,
    pub members: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Online,
    Offline,
    Busy,
    Away,
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
        let mut messages = HashMap::new();
        // 为alice初始化一些消息
        messages.insert("alice".to_string(), vec![
            Message::new("alice".to_string(), "Hello there!".to_string(), false),
            Message::new("You".to_string(), "Hi Alice, how are you?".to_string(), true),
        ]);
        // 为dev-team初始化一些消息
        messages.insert("dev-team".to_string(), vec![
            Message::new("bob".to_string(), "Hey team, let's meet at 2pm".to_string(), false),
            Message::new("alice".to_string(), "Sounds good to me".to_string(), false),
        ]);
        
        Self {
            input: String::new(),
            messages,
            contacts: vec![
                Contact { name: "alice".to_string(), status: Status::Online },
                Contact { name: "bob".to_string(), status: Status::Offline },
            ],
            groups: vec![
                Group { name: "dev-team".to_string(), members: vec!["alice".to_string(), "bob".to_string()] },
                Group { name: "random".to_string(), members: vec!["alice".to_string()] },
            ],
            current_view: View::Chat {
                target: "alice".to_string(),
            },
            mode: Mode::Normal,
            scroll_offset: 0,
            selected_contact: None,
            selected_group: None,
            current_user: "user1".to_string(),
            chat_maximized: false, // 初始化最大化状态
            token: None, // 初始化token为空
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

    pub fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) -> bool {
        let mut should_exit = false;
        match self.mode {
            Mode::Normal => match key.code {
                crossterm::event::KeyCode::Char('q') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => {
                    // Ctrl+Q 退出
                    should_exit = true;
                }
                crossterm::event::KeyCode::Char('i') => {
                    self.mode = Mode::Insert;
                }
                crossterm::event::KeyCode::Char('k') => {
                    // 在联系人或群组视图中向上导航
                    match self.current_view {
                        View::Contacts => {
                            if !self.contacts.is_empty() {
                                if let Some(selected) = self.selected_contact {
                                    self.selected_contact = Some(selected.saturating_sub(1));
                                } else {
                                    self.selected_contact = Some(0);
                                }
                            }
                        }
                        View::Groups => {
                            if !self.groups.is_empty() {
                                if let Some(selected) = self.selected_group {
                                    self.selected_group = Some(selected.saturating_sub(1));
                                } else {
                                    self.selected_group = Some(0);
                                }
                            }
                        }
                        _ => {
                            // 在聊天视图中，k键用于滚动消息
                            if self.scroll_offset > 0 {
                                self.scroll_offset -= 1;
                            }
                        }
                    }
                }
                crossterm::event::KeyCode::Char('j') => {
                    // 在联系人或群组视图中向下导航
                    match self.current_view {
                        View::Contacts => {
                            if !self.contacts.is_empty() {
                                if let Some(selected) = self.selected_contact {
                                    self.selected_contact = Some((selected + 1).min(self.contacts.len() - 1));
                                } else {
                                    self.selected_contact = Some(0);
                                }
                            }
                        }
                        View::Groups => {
                            if !self.groups.is_empty() {
                                if let Some(selected) = self.selected_group {
                                    self.selected_group = Some((selected + 1).min(self.groups.len() - 1));
                                } else {
                                    self.selected_group = Some(0);
                                }
                            }
                        }
                        _ => {
                            // 在聊天视图中，j键用于滚动消息
                            self.scroll_offset += 1;
                        }
                    }
                }
                crossterm::event::KeyCode::Char('h') => {
                    // 切换到联系人视图
                    self.current_view = View::Contacts;
                }
                crossterm::event::KeyCode::Char('g') => {
                    // 切换到群组视图
                    self.current_view = View::Groups;
                }
                crossterm::event::KeyCode::Char('m') => {
                    // 切换聊天窗口最大化状态
                    if matches!(self.current_view, View::Chat { .. }) {
                        self.chat_maximized = !self.chat_maximized;
                    }
                }
                crossterm::event::KeyCode::Enter => {
                    // 在联系人或群组视图中按Enter选择
                    match &self.current_view {
                        View::Contacts => {
                            if let Some(index) = self.selected_contact {
                                if index < self.contacts.len() {
                                    let target = self.contacts[index].name.clone();
                                    self.current_view = View::Chat { target: target.clone() };
                                    
                                    // 确保目标有消息列表
                                    if !self.messages.contains_key(&target) {
                                        self.messages.insert(target.clone(), vec![]);
                                    }
                                }
                            }
                        }
                        View::Groups => {
                            if let Some(index) = self.selected_group {
                                if index < self.groups.len() {
                                    let target = self.groups[index].name.clone();
                                    self.current_view = View::Chat { target: target.clone() };
                                    
                                    // 确保目标有消息列表
                                    if !self.messages.contains_key(&target) {
                                        self.messages.insert(target.clone(), vec![]);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                crossterm::event::KeyCode::Tab => {
                    // 在不同视图间切换
                    self.current_view = match self.current_view {
                        View::Chat { .. } => View::Contacts,
                        View::Contacts => View::Groups,
                        View::Groups => View::Chat { target: "alice".to_string() },
                    };
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
        should_exit
    }

    fn send_message(&mut self) -> bool {
        if self.input.is_empty() {
            return false;
        }

        // 处理命令
        let should_exit = if self.input.starts_with('/') {
            self.handle_command()
        } else {
            // 发送普通消息
            match &self.current_view {
                View::Chat { target } => {
                    // 确保目标有消息列表
                    if !self.messages.contains_key(target) {
                        self.messages.insert(target.clone(), vec![]);
                    }
                    
                    // 添加发送的消息
                    if let Some(messages) = self.messages.get_mut(target) {
                        let msg = Message::new(
                            "You".to_string(), 
                            self.input.clone(), 
                            true
                        );
                        messages.push(msg);
                    }
                    
                    // 同时模拟接收消息（用于演示）
                    // 在真实应用中，这将来自网络
                    if self.contacts.iter().any(|c| c.name == *target) {
                        // 这是发送给联系人的消息
                        if let Some(messages) = self.messages.get_mut(target) {
                            let response = Message::new(
                                target.clone(),
                                format!("Thanks for your message: \"{}\"", self.input),
                                false
                            );
                            messages.push(response);
                        }
                    } else if self.groups.iter().any(|g| g.name == *target) {
                        // 这是发送给群组的消息
                        if let Some(messages) = self.messages.get_mut(target) {
                            let response = Message::new(
                                "bot".to_string(),
                                format!("Message received in {}: \"{}\"", target, self.input),
                                false
                            );
                            messages.push(response);
                        }
                    }
                },
                _ => {
                    // 不在聊天视图中，无法发送消息
                    // 创建一个临时消息向量来显示系统消息
                    let system_target = "system".to_string();
                    if !self.messages.contains_key(&system_target) {
                        self.messages.insert(system_target.clone(), vec![]);
                    }
                    if let Some(messages) = self.messages.get_mut(&system_target) {
                        let msg = Message::system("Cannot send message: not in chat view");
                        messages.push(msg);
                    }
                }
            };
            // TODO: 实际发送到网络
            // self.network.send(MessagePacket { ... });
            false
        };

        self.input.clear();
        self.mode = Mode::Normal;
        should_exit
    }

    fn handle_command(&mut self) -> bool {
        let mut should_exit = false;
        let cmd = self.input.split_whitespace().next().unwrap_or("");
        // 确定消息应该添加到哪个目标
        let target = match &self.current_view {
            View::Chat { target } => target.clone(),
            _ => "system".to_string(),
        };
        
        // 确保目标有消息列表
        if !self.messages.contains_key(&target) {
            self.messages.insert(target.clone(), vec![]);
        }
        
        match cmd {
            "/help" => {
                if let Some(messages) = self.messages.get_mut(&target) {
                    messages.push(Message::system("Available commands:"));
                    messages.push(Message::system("/help - Show this help"));
                    messages.push(Message::system("/clear - Clear chat history"));
                    messages.push(Message::system("/quit or /exit - Exit the application"));
                    messages.push(Message::system("/list - List contacts and groups"));
                    messages.push(Message::system("/join <group> - Join a group"));
                    messages.push(Message::system("/create <group> - Create a new group"));
                    messages.push(Message::system("/status <status> - Change your status"));
                }
            }
            "/clear" => {
                if let Some(messages) = self.messages.get_mut(&target) {
                    messages.clear();
                }
            }
            "/quit" | "/exit" => {
                should_exit = true;
            }
            "/list" => {
                if let Some(messages) = self.messages.get_mut(&target) {
                    let contact_list = self.contacts.iter()
                        .map(|c| format!("{} ({})", c.name, match c.status {
                            Status::Online => "online",
                            Status::Offline => "offline",
                            Status::Busy => "busy",
                            Status::Away => "away",
                        }))
                        .collect::<Vec<_>>()
                        .join(", ");
                    let group_list = self.groups.iter()
                        .map(|g| g.name.clone())
                        .collect::<Vec<_>>()
                        .join(", ");
                    messages.push(Message::system(&format!("Contacts: {}", contact_list)));
                    messages.push(Message::system(&format!("Groups: {}", group_list)));
                }
            }
            "/join" => {
                let parts: Vec<&str> = self.input.split_whitespace().collect();
                if parts.len() >= 2 {
                    let group_name = parts[1];
                    // 检查群组是否存在
                    if self.groups.iter().any(|g| g.name == group_name) {
                        self.current_view = View::Chat { target: group_name.to_string() };
                        // 确保目标有消息列表
                        if !self.messages.contains_key(group_name) {
                            self.messages.insert(group_name.to_string(), vec![]);
                        }
                        if let Some(messages) = self.messages.get_mut(group_name) {
                            messages.push(Message::system(&format!("Joined group: {}", group_name)));
                        }
                    } else {
                        if let Some(messages) = self.messages.get_mut(&target) {
                            messages.push(Message::system(&format!("Group '{}' not found", group_name)));
                        }
                    }
                } else {
                    if let Some(messages) = self.messages.get_mut(&target) {
                        messages.push(Message::system("Usage: /join <group>"));
                    }
                }
            }
            "/create" => {
                let parts: Vec<&str> = self.input.split_whitespace().collect();
                if parts.len() >= 2 {
                    let group_name = parts[1];
                    // 检查群组是否已存在
                    if self.groups.iter().any(|g| g.name == group_name) {
                        if let Some(messages) = self.messages.get_mut(&target) {
                            messages.push(Message::system(&format!("Group '{}' already exists", group_name)));
                        }
                    } else {
                        self.groups.push(Group {
                            name: group_name.to_string(),
                            members: vec!["user1".to_string()], // 当前用户
                        });
                        if let Some(messages) = self.messages.get_mut(&target) {
                            messages.push(Message::system(&format!("Created group: {}", group_name)));
                        }
                        // 为新群组初始化消息列表
                        self.messages.insert(group_name.to_string(), vec![]);
                    }
                } else {
                    if let Some(messages) = self.messages.get_mut(&target) {
                        messages.push(Message::system("Usage: /create <group>"));
                    }
                }
            }
            "/status" => {
                let parts: Vec<&str> = self.input.split_whitespace().collect();
                if parts.len() >= 2 {
                    let status_str = parts[1];
                    if let Some(messages) = self.messages.get_mut(&target) {
                        messages.push(Message::system(&format!("Status changed to: {}", status_str)));
                    }
                    // TODO: 实际更改状态
                } else {
                    if let Some(messages) = self.messages.get_mut(&target) {
                        messages.push(Message::system("Usage: /status <status>"));
                    }
                }
            }
            _ => {
                if let Some(messages) = self.messages.get_mut(&target) {
                    messages.push(Message::system(&format!("Unknown command: {}", cmd)));
                    messages.push(Message::system("Type /help for available commands"));
                }
            }
        }
        self.input.clear();
        self.mode = Mode::Normal;
        should_exit
    }

    pub fn render(&self, frame: &mut Frame) {
        let size = frame.size();
        match &self.current_view {
            View::Chat { target } => {
                if self.chat_maximized {
                    self.render_maximized_chat_layout(frame, size, target)
                } else {
                    self.render_main_layout(frame, size, target)
                }
            },
            View::Contacts => self.render_contacts_layout(frame, size),
            View::Groups => self.render_groups_layout(frame, size),
        }
    }

    fn render_maximized_chat_layout(&self, frame: &mut Frame, area: Rect, target: &str) {
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

    fn render_main_layout(&self, frame: &mut Frame, area: Rect, target: &str) {
        // 三栏布局：联系人列表(1/4) + 聊天窗口(1/2) + 群组列表(1/4)
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20), // 联系人列表
                Constraint::Percentage(60), // 聊天窗口
                Constraint::Percentage(20), // 群组列表
            ])
            .split(area);

        // 左侧联系人列表
        self.render_contacts_list(frame, chunks[0]);

        // 中间聊天区域
        let chat_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),         // 消息区域
                Constraint::Length(5),      // 增大输入框区域
            ])
            .split(chunks[1]);

        self.render_messages(frame, chat_chunks[0]);
        self.render_input(frame, chat_chunks[1]);

        // 右侧群组列表
        self.render_groups_list(frame, chunks[2]);
    }

    fn render_contacts_layout(&self, frame: &mut Frame, area: Rect) {
        // 主要显示联系人列表，带一些聊天区域
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30), // 联系人列表
                Constraint::Percentage(70), // 信息区域
            ])
            .split(area);

        self.render_contacts_list(frame, chunks[0]);
        
        // 右侧显示联系人详细信息或帮助
        let info_block = Block::default()
            .title("Contact Info")
            .borders(Borders::ALL);
            
        let info_text = if let Some(index) = self.selected_contact {
            if index < self.contacts.len() {
                let contact = &self.contacts[index];
                format!("Name: {}\nStatus: {}\n\nPress Enter to chat", 
                    contact.name,
                    match contact.status {
                        Status::Online => "Online",
                        Status::Offline => "Offline",
                        Status::Busy => "Busy",
                        Status::Away => "Away",
                    })
            } else {
                "Select a contact".to_string()
            }
        } else {
            "Select a contact".to_string()
        };
        
        let info = Paragraph::new(info_text)
            .block(info_block);
            
        frame.render_widget(info, chunks[1]);
    }

    fn render_groups_layout(&self, frame: &mut Frame, area: Rect) {
        // 主要显示群组列表，带一些信息区域
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30), // 群组列表
                Constraint::Percentage(70), // 信息区域
            ])
            .split(area);

        self.render_groups_list(frame, chunks[0]);
        
        // 右侧显示群组详细信息或帮助
        let info_block = Block::default()
            .title("Group Info")
            .borders(Borders::ALL);
            
        let info_text = if let Some(index) = self.selected_group {
            if index < self.groups.len() {
                let group = &self.groups[index];
                format!("Name: {}\nMembers: {}\n\nPress Enter to chat", 
                    group.name,
                    group.members.join(", "))
            } else {
                "Select a group".to_string()
            }
        } else {
            "Select a group".to_string()
        };
        
        let info = Paragraph::new(info_text)
            .block(info_block);
            
        frame.render_widget(info, chunks[1]);
    }

    fn render_contacts_list(&self, frame: &mut Frame, area: Rect) {
        let contacts: Vec<ListItem> = self.contacts
            .iter()
            .enumerate()
            .map(|(i, contact)| {
                let status_char = match contact.status {
                    Status::Online => "🟢",
                    Status::Offline => "🔴",
                    Status::Busy => "🔴",
                    Status::Away => "🟡",
                };
                let content = format!("{} {}", status_char, contact.name);
                let mut item = ListItem::new(content);
                if let Some(selected) = self.selected_contact {
                    if selected == i {
                        item = item.style(Style::default().bg(Color::Blue));
                    }
                }
                item
            })
            .collect();

        let title = match self.current_view {
            View::Contacts => "Contacts (↑/↓ to select)",
            _ => "Contacts"
        };

        let contacts_list = List::new(contacts)
            .block(Block::default().title(title).borders(Borders::ALL));

        frame.render_widget(contacts_list, area);
    }

    fn render_groups_list(&self, frame: &mut Frame, area: Rect) {
        let groups: Vec<ListItem> = self.groups
            .iter()
            .enumerate()
            .map(|(i, group)| {
                let content = format!("👥 {}", group.name);
                let mut item = ListItem::new(content);
                if let Some(selected) = self.selected_group {
                    if selected == i {
                        item = item.style(Style::default().bg(Color::Blue));
                    }
                }
                item
            })
            .collect();

        let title = match self.current_view {
            View::Groups => "Groups (↑/↓ to select)",
            _ => "Groups"
        };

        let groups_list = List::new(groups)
            .block(Block::default().title(title).borders(Borders::ALL));

        frame.render_widget(groups_list, area);
    }

    fn render_messages(&self, frame: &mut Frame, area: Rect) {
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
                // 检查目标是联系人还是群组
                if self.contacts.iter().any(|c| c.name == *target) {
                    format!("Chat with {} (Contact) {}", target, 
                        if self.chat_maximized { "[M] (Press 'm' to restore)" } else { "[M] (Press 'm' to maximize)" })
                } else if self.groups.iter().any(|g| g.name == *target) {
                    format!("Chat in {} (Group) {}", target,
                        if self.chat_maximized { "[M] (Press 'm' to restore)" } else { "[M] (Press 'm' to maximize)" })
                } else {
                    format!("Chat with {} {}", target,
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

    fn render_input(&self, frame: &mut Frame, area: Rect) {
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
            frame.set_cursor(
                chunks[0].x + self.input.len() as u16 + 1,
                chunks[0].y + 1,
            );
        }
    }

    /// 获取当前聊天目标的详细信息
    pub fn get_current_target_info(&self) -> String {
        match &self.current_view {
            View::Chat { target } => {
                // 检查是否是联系人
                if let Some(contact) = self.contacts.iter().find(|c| c.name == *target) {
                    format!(
                        "👤 {} ({})",
                        contact.name,
                        match contact.status {
                            Status::Online => "🟢 Online",
                            Status::Offline => "🔴 Offline",
                            Status::Busy => "🔴 Busy",
                            Status::Away => "🟡 Away",
                        }
                    )
                } 
                // 检查是否是群组
                else if let Some(group) = self.groups.iter().find(|g| g.name == *target) {
                    format!(
                        "👥 {} ({} members)",
                        group.name,
                        group.members.len()
                    )
                } 
                // 默认情况
                else {
                    format!("💬 {}", target)
                }
            },
            View::Contacts => "📋 Contacts".to_string(),
            View::Groups => "👥 Groups".to_string(),
        }
    }
}