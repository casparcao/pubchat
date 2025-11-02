use crate::model::contact::Contact;
use crate::model::user::User;
use crate::repository::db;
use anyhow::Result;

pub async fn insert(contact: Contact) -> Result<i64> {
    let mut connection = db::connection().await?;
    let result = sqlx::query("INSERT INTO contact (id, here, there, status) VALUES (?, ?, ?, ?)")
        .bind(contact.id)
        .bind(contact.here)
        .bind(contact.there)
        .bind(contact.status)
        .execute(connection.as_mut())
        .await?;

    Ok(result.last_insert_id() as i64)
}

pub async fn select_by_here(here: i64) -> Result<Vec<User>> {
    let mut connection = db::connection().await?;
    let friends = sqlx::query_as(
        "SELECT u.id, u.name, u.password, cast(u.gender as signed) as gender, \
        u.age, u.createtime, u.open_id, u.union_id, u.avatar \
        FROM user u \
        INNER JOIN contact f ON u.id = f.there \
        WHERE f.here = ? AND f.status = 1"
    )
    .bind(here)
    .fetch_all(connection.as_mut())
    .await?;
    log::info!("select_by_here: {}", friends.len());
    Ok(friends)
}

pub async fn select_by_ids(here: i64, there: i64) -> Result<Option<Contact>> {
    let mut connection = db::connection().await?;
    let friend = sqlx::query_as(
        "SELECT id, here, there, status, createtime, updatetime \
        FROM contact \
        WHERE here = ? AND there = ?"
    )
    .bind(here)
    .bind(there)
    .fetch_optional(connection.as_mut())
    .await?;

    Ok(friend)
}

pub async fn delete(here: i64, there: i64) -> Result<u64> {
    let mut connection = db::connection().await?;
    let result = sqlx::query(
        "DELETE FROM contact \
        WHERE here = ? AND there = ?"
    )
    .bind(here)
    .bind(there)
    .execute(connection.as_mut())
    .await?;

    Ok(result.rows_affected())
}