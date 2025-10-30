use core::response::ApiErr;

use anyhow::{Ok, Result};
use crate::vo::session::{CreateSessionRequest, SessionDetailResponse, UserSessionResponse};
use crate::model::session::{Session, UserSession};
use crate::repository::session as session_repo;
use chrono::Utc;

pub async fn create_session(creator_id: i64, payload: CreateSessionRequest) -> Result<Session> {
    if payload.id <= 0 {
        return Err(ApiErr::Bad(400, "会话ID不能小于等于0".to_string()).into());
    }
    let existed : Option<Session> = session_repo::find_session_by_id(payload.id).await?;
    if existed.is_some() {
        return Ok(existed.unwrap());
    }
    let now = Utc::now().naive_utc();
    let session_id = payload.id;
    // 创建会话
    let session = Session {
        id: session_id,
        name: payload.name,
        creator: creator_id,
        createtime: now,
        updatetime: now,
    };
    
    // 保存会话
    session_repo::create_session(&session).await?;
    
    // 添加会话成员（包括创建者）
    for member in payload.members {
        let user_session = UserSession {
            id: snowflaker::next_id()? as i64,
            user_id: member.id,
            user_name: member.name,
            session_id: session.id,
            role: if member.id == creator_id { 1 } else { 0 }, // 创建者为管理员
            jointime: now,
        };
        session_repo::create_user_session(&user_session).await?;
    }
    Ok(session)
}

pub async fn get_sessions_by_user(user_id: i64) -> Result<Vec<Session>> {
    session_repo::find_sessions_by_user(user_id).await
}

pub async fn get_session_by_id(session_id: i64) -> Result<SessionDetailResponse> {
    let session: Option<Session> = session_repo::find_session_by_id(session_id).await?;
    if session.is_none() {
        return Err(ApiErr::Bad(404, "会话不存在".to_string()).into());
    }
    let session = session.unwrap();
    // 获取会话成员列表
    let members = session_repo::find_user_sessions_by_session(session_id).await?;
    let detail = SessionDetailResponse {
        id: session.id,
        name: session.name,
        members: members.into_iter().map(|m| UserSessionResponse {
            id: m.user_id,
            name: m.user_name,
        }).collect(),
    };
    Ok(detail)
}
