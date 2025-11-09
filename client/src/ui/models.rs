use std::sync::Arc;

use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::Mutex;

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
            timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
            system,
        }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct Contact {
    pub id: i64,
    pub name: String,
    pub status: Status,
    pub avatar: Option<String>,
}

impl From<core::api::types::contact::ContactResponse> for Contact {
    fn from(contact: core::api::types::contact::ContactResponse) -> Self {
        Self {
            id: contact.id,
            name: contact.name,
            status: Status::Offline, // 默认状态为离线
            avatar: contact.avatar,
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Session {
    pub id: i64,
    pub name: String,
    pub members: Vec<Contact>,
}

impl From<core::api::types::session::SessionResponse> for Session {
    fn from(session: core::api::types::session::SessionResponse) -> Self {
        Self {
            id: session.id,
            name: session.name,
            members: vec![],
        }
    }
}

impl From<core::api::types::session::SessionDetailResponse> for Session {
    fn from(session: core::api::types::session::SessionDetailResponse) -> Self {
        let mut members = vec![];
        for member in session.members {
            members.push(Contact::from(member));
        }
        Self {
            id: session.id,
            name: session.name,
            members: members,
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Online,
    Offline,
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
    // 添加token字段存储用户认证信息
    pub token: String,
    pub stream: Arc<Mutex<OwnedWriteHalf>>,
    pub me: Me,
}