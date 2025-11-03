use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::tcp::OwnedWriteHalf;

use crate::ui::screen::chat::ChatScreen;
use crate::ui::screen::contact::ContactListScreen;


#[derive(Debug, Clone)]
pub struct Message {
    pub sender: String,
    pub content: String,
    pub timestamp: String,
    //是否是系统消息
    pub system: bool,
}

impl Message {
    pub fn new(sender: String, content: String, system: bool) -> Self {
        Self {
            sender,
            content,
            timestamp: chrono::Local::now().format("%H:%M").to_string(),
            system,
        }
    }

    pub fn system(content: &str) -> Self {
        Self::new("SYSTEM".to_string(), content.to_string(), true)
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
    pub fn from_friend_response(friend: crate::remote::contact::ContactResponse) -> Self {
        Self {
            id: friend.id,
            name: friend.name,
            status: Status::Offline, // 默认状态为离线
            avatar: friend.avatar,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Session {
    pub id: i64,
    pub name: String,
}

impl Session {
    pub fn from_session_response(session: crate::remote::session::SessionResponse) -> Self {
        Self {
            id: session.id,
            name: session.name,
        }
    }
}

impl Default for Session {
    fn default() -> Self {
        Self {
            id: 0,
            name: "None".to_string(),
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
    Chat,
    Contact, // 添加好友列表视图
}

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Normal,
    Insert,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Me {
    pub id: u64,
    pub name: String,
}

// 应用状态
#[derive(Debug, Clone)]
pub struct App {
    // 联系人列表组件渲染逻辑
    pub contact: ContactListScreen,
    // 会话列表组件渲染逻辑
    pub chat: ChatScreen,
    //当前页面处于哪个视图
    pub view: View,
    pub me: Me,
    // 添加token字段存储用户认证信息
    pub token: String,
    // 添加TCP流用于发送消息
    pub stream: Arc<Mutex<OwnedWriteHalf>>,
}