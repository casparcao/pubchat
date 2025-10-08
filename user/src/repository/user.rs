use crate::model::user::User;
use crate::repository::db;
use anyhow::Result;

pub async fn select_user_by_name(name: &str) -> Result<Option<User>> {
    let mut connection = db::connection().await?;
    let user
        = sqlx::query_as("SELECT id, name, password, cast(gender as signed) as gender, \
            age, createtime, open_id, union_id, avatar \
            FROM user WHERE name = ?")
        .bind(name)
        .fetch_optional(connection.as_mut())
        .await?;
    Ok(user)
}

pub async fn select_user_by_open_id(open_id: &str) -> Result<Option<User>> {
    let mut connection = db::connection().await?;
    let user 
        = sqlx::query_as("SELECT id, name, password, cast(gender as signed) as gender, \
            age, createtime, open_id, union_id, avatar \
            FROM user WHERE open_id = ?")
        .bind(open_id)
        .fetch_optional(connection.as_mut())
        .await?;
    Ok(user)
}

pub async fn insert_user(user: User) -> Result<i64> {
    let mut connection = db::connection().await?;
    sqlx::query("INSERT INTO user (id, name, password, gender, age, open_id, union_id, avatar) VALUES (?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(user.id)
        .bind(user.name)
        .bind(user.password)
        .bind(user.gender.to_string())
        .bind(user.age)
        .bind(user.open_id)
        .bind(user.union_id)
        .bind(user.avatar)
        .execute(connection.as_mut())
        .await?;
    Ok(user.id)
}

pub async fn update_avatar<'a>(id: i64, avatar: &str) -> Result<()>{
    let mut connection = db::connection().await?;
    sqlx::query("UPDATE user SET avatar = ? WHERE id = ?")
        .bind(avatar)
        .bind(id)
        .execute(connection.as_mut())
        .await?;
    Ok(())
}

pub(crate) async fn select_user_by_id(id: i64) -> Result<Option<User>>{
    let mut connection = db::connection().await?;
    let user
        = sqlx::query_as("SELECT id, name, password, cast(gender as signed) as gender, \
            age, createtime, open_id, union_id, avatar \
            FROM user WHERE id = ?")
        .bind(id)
        .fetch_optional(connection.as_mut())
        .await?;
    Ok(user)
}

pub(crate) async fn update_username(id: i64, name: String) -> Result<()>{
    let mut connection = db::connection().await?;
    sqlx::query("UPDATE user SET name = ? WHERE id = ?")
        .bind(name)
        .bind(id)
        .execute(connection.as_mut())
        .await?;
    Ok(())
}