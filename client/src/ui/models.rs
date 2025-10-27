use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::tcp::OwnedWriteHalf;

#[derive(Debug, Clone)]
pub struct MessageItem {
    pub sender: String,
    pub content: String,
    pub timestamp: String,
    pub is_user: bool,
}

impl MessageItem {
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

#[derive(Debug, Clone)]
pub struct Contact {
    pub id: i64,
    pub name: String,
    pub status: Status,
    pub avatar: Option<String>,
}

impl Contact {
    pub fn new(id: i64, name: String, status: Status, avatar: Option<String>) -> Self {
        Self { id, name, status, avatar }
    }
    
    // 从好友响应创建联系人
    pub fn from_friend_response(friend: crate::ui::friend_service::FriendResponse) -> Self {
        Self {
            id: friend.id,
            name: friend.name,
            status: Status::Offline, // 默认状态为离线
            avatar: friend.avatar,
        }
    }
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
}

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Normal,
    Insert,
}

// 应用状态
#[derive(Debug, Clone)]
pub struct App {
    pub input: String,
    pub messages: HashMap<String, Vec<MessageItem>>,
    pub contacts: Vec<Contact>,
    pub current_view: View,
    pub mode: Mode,
    pub scroll_offset: u16,
    pub selected_contact: Option<usize>,
    pub current_user: String,
    pub current_user_id: u64,
    pub chat_maximized: bool,
    // 添加token字段存储用户认证信息
    pub token: Option<String>,
    // 添加TCP流用于发送消息
    pub stream: Option<Arc<Mutex<OwnedWriteHalf>>>,
}