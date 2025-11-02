use anyhow::Result;
use sqlx::{QueryBuilder, Sqlite, prelude::FromRow};
use crate::db;

#[derive(Debug, Clone, FromRow)]
pub struct Session {
    pub id: i64,
    pub uid: i64,
    pub sid: i64,
    pub name: String,
    pub avatar: Option<String>,
}

// 查询好友列表
pub async fn select(uid: i64) -> Result<Vec<Session>> {
    let mut connection = db::connection().await?;
    let mut builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        "SELECT id, uid, sid, name, avatar FROM session where uid = ");
    builder.push_bind(uid);
    let sessions: Vec<Session> = builder
        .build_query_as()
        .fetch_all(connection.as_mut())
        .await?;
    Ok(sessions)
}

// 保存好友列表
pub async fn save(uid: i64, sessions: &[Session]) -> Result<()> {
    let mut connection = db::connection().await?;
    // 先清空表
    sqlx::query("DELETE FROM session where uid = ?")
        .bind(uid)
        .execute(connection.as_mut())
        .await?;
    
    // 批量插入好友
    for session in sessions {
        sqlx::query("INSERT INTO session (id, uid, sid, name, avatar) VALUES (?, ?, ?, ?, ?)")
            .bind(snowflaker::next_id().unwrap() as i64)
            .bind(uid)
            .bind(session.id)
            .bind(&session.name)
            .bind(&session.avatar)
            .execute(connection.as_mut())
            .await?;
    }
    Ok(())
}