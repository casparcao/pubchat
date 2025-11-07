use anyhow::Result;
use crate::model::session::{Session, UserSession};
use crate::repository::db;

pub async fn create_session(session: &Session) -> Result<Session> {
    let mut connection = db::connection().await?;
    // 插入会话记录
    sqlx::query("INSERT INTO sessions (id, name, creator, createtime, updatetime) VALUES (?, ?, ?, ?, ?)")
        .bind(&session.id)
        .bind(&session.name)
        .bind(&session.creator)
        .bind(&session.createtime)
        .bind(&session.updatetime)
        .execute(connection.as_mut())
        .await?;
        
    Ok(session.clone())
}

pub async fn create_user_session(user_session: &UserSession) -> Result<UserSession> {
    let mut connection = db::connection().await?;
    
    // 插入用户会话关联记录
    sqlx::query("INSERT INTO user_sessions (id, uid, uname, sid, role, jointime) VALUES (?, ?, ?, ?, ?, ?)")
        .bind(&user_session.id)
        .bind(&user_session.uid)
        .bind(&user_session.uname)
        .bind(&user_session.sid)
        .bind(&user_session.role)
        .bind(&user_session.jointime)
        .execute(connection.as_mut())
        .await?;
        
    Ok(user_session.clone())
}

pub async fn find_session_by_id(id: i64) -> Result<Option<Session>> {
    let mut connection = db::connection().await?;
    let result = sqlx::query_as::<_, Session>("SELECT * FROM sessions WHERE id = ?")
        .bind(id)
        .fetch_optional(connection.as_mut())
        .await?;
    Ok(result)
}

pub async fn find_sessions_by_user(user_id: i64) -> Result<Vec<Session>> {
    let mut connection = db::connection().await?;
    let result = sqlx::query_as::<_, Session>(
        r#"
        SELECT s.* FROM sessions s 
        JOIN user_sessions us ON s.id = us.sid 
        WHERE us.uid = ?
        ORDER BY s.updatetime DESC
        "#
    )
    .bind(user_id)
    .fetch_all(connection.as_mut())
    .await?;
    Ok(result)
}

pub async fn find_user_sessions_by_session(sid: i64) -> Result<Vec<UserSession>> {
    let mut connection = db::connection().await?;
    let result = sqlx::query_as::<_, UserSession>("SELECT * FROM user_sessions WHERE sid = ?")
        .bind(sid)
        .fetch_all(connection.as_mut())
        .await?;
    Ok(result)
}
