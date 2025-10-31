use core::response::ApiErr;

use crate::ui::models::{App, Session as AppSession};
use crate::service::session::{create_session, CreateSessionRequest, CreateSessionUserRequest};

impl App {
    /// 为两个用户创建或获取会话
    pub fn create_or_get_session(&mut self, friend_name: String, friend_id: i64) -> anyhow::Result<AppSession> {
        if let Some(ref token) = self.token {
            // 计算会话ID
            let session_id = crate::service::session::calc_session_id(self.current_user_id as i64, friend_id) as i64;
            // 构造创建会话请求
            let request = CreateSessionRequest {
                id: session_id,
                name: format!("{} and {}", self.current_user, friend_name),
                members: vec![
                    CreateSessionUserRequest {
                        id: self.current_user_id as i64,
                        name: self.current_user.clone(),
                    },
                    CreateSessionUserRequest {
                        id: friend_id,
                        name: friend_name.clone(),
                    }
                ],
            };
            // 创建会话
            match create_session(token, request){
                Ok(session_response) => {
                    // 创建或更新本地会话列表
                    let app_session = AppSession::from_session_response(session_response);
                    Ok(app_session)
                }
                Err(e) => {
                    Err(ApiErr::Error(format!("Failed to create session: {}", e)).into())
                }
            }
        } else {
            Err(ApiErr::Error(format!("User not authenticated")).into())
        }
    }
    
}