use anyhow::Result;
use sqlx::{QueryBuilder, Sqlite, prelude::FromRow};
use crate::db;
use crate::service::friend::FriendResponse;

#[derive(Debug, Clone, FromRow)]
pub struct Friend {
    pub id: i64,
    pub name: String,
    pub avatar: Option<String>,
}

// 查询好友列表
pub async fn select_friends() -> Result<Vec<FriendResponse>> {
    let mut connection = db::connection().await?;
    let mut builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        "SELECT id, name, avatar FROM friends ORDER BY name");
    
    let friends: Vec<FriendResponse> = builder
        .build_query_as()
        .fetch_all(connection.as_mut())
        .await?;
        
    Ok(friends)
}

// 保存好友列表
pub async fn save_friends(friends: &[FriendResponse]) -> Result<()> {
    let mut connection = db::connection().await?;
    
    // 先清空表
    sqlx::query("DELETE FROM friends")
        .execute(connection.as_mut())
        .await?;
    
    // 批量插入好友
    for friend in friends {
        sqlx::query("INSERT INTO friends (id, name, avatar) VALUES (?, ?, ?)")
            .bind(friend.id)
            .bind(&friend.name)
            .bind(&friend.avatar)
            .execute(connection.as_mut())
            .await?;
    }
    
    Ok(())
}