use anyhow::Result;
use sqlx::{QueryBuilder, Sqlite, prelude::FromRow};
use crate::db;
use crate::service::contact::ContactResponse;

#[derive(Debug, Clone, FromRow)]
pub struct Contact {
    pub id: i64,
    pub here: i64,
    pub there: i64,
    pub name: String,
    pub avatar: Option<String>,
}

// 查询好友列表
pub async fn select(uid: i64) -> Result<Vec<ContactResponse>> {
    let mut connection = db::connection().await?;
    let mut builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        "SELECT id, here, there, name, avatar FROM contact where here = ");
    builder.push_bind(uid);
    let contacts: Vec<Contact> = builder
        .build_query_as()
        .fetch_all(connection.as_mut())
        .await?;
    let contacts = contacts.into_iter().map(|contact| ContactResponse {
        id: contact.id,
        name: contact.name,
        avatar: contact.avatar,
    }).collect::<Vec<_>>(); 
    Ok(contacts)
}

// 保存好友列表
pub async fn save(uid: i64, contacts: &[ContactResponse]) -> Result<()> {
    let mut connection = db::connection().await?;
    // 先清空表
    sqlx::query("DELETE FROM contact where here = ?")
        .bind(uid)
        .execute(connection.as_mut())
        .await?;
    
    // 批量插入好友
    for contact in contacts {
        sqlx::query("INSERT INTO contact (id, here, there, name, avatar) VALUES (?, ?, ?, ?, ?)")
            .bind(snowflaker::next_id().unwrap() as i64)
            .bind(uid)
            .bind(contact.id)
            .bind(&contact.name)
            .bind(&contact.avatar)
            .execute(connection.as_mut())
            .await?;
    }
    Ok(())
}