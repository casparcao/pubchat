use crate::model::friend::Friend;
use crate::model::user::User;
use crate::repository::db;
use anyhow::Result;

pub async fn insert_friend(friend: Friend) -> Result<i64> {
    let mut connection = db::connection().await?;
    let result = sqlx::query("INSERT INTO friend (id, user_id, friend_id, status) VALUES (?, ?, ?, ?)")
        .bind(friend.id)
        .bind(friend.user_id)
        .bind(friend.friend_id)
        .bind(friend.status)
        .execute(connection.as_mut())
        .await?;

    Ok(result.last_insert_id() as i64)
}

pub async fn select_friends_by_user_id(user_id: i64) -> Result<Vec<User>> {
    let mut connection = db::connection().await?;
    let friends = sqlx::query_as(
        "SELECT u.id, u.name, u.password, cast(u.gender as signed) as gender, \
        u.age, u.createtime, u.open_id, u.union_id, u.avatar \
        FROM user u \
        INNER JOIN friend f ON u.id = f.friend_id \
        WHERE f.user_id = ? AND f.status = 1"
    )
    .bind(user_id)
    .fetch_all(connection.as_mut())
    .await?;

    Ok(friends)
}

pub async fn select_friend_by_ids(user_id: i64, friend_id: i64) -> Result<Option<Friend>> {
    let mut connection = db::connection().await?;
    let friend = sqlx::query_as(
        "SELECT id, user_id, friend_id, status, createtime, updatetime \
        FROM friend \
        WHERE user_id = ? AND friend_id = ?"
    )
    .bind(user_id)
    .bind(friend_id)
    .fetch_optional(connection.as_mut())
    .await?;

    Ok(friend)
}

pub async fn delete_friend(user_id: i64, friend_id: i64) -> Result<u64> {
    let mut connection = db::connection().await?;
    let result = sqlx::query(
        "DELETE FROM friend \
        WHERE user_id = ? AND friend_id = ?"
    )
    .bind(user_id)
    .bind(friend_id)
    .execute(connection.as_mut())
    .await?;

    Ok(result.rows_affected())
}