use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{NaiveDateTime};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: i64,
    pub name: String,
    pub creator: i64,
    pub createtime: NaiveDateTime,
    pub updatetime: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserSession {
    pub id: i64,
    pub uid: i64,
    pub uname: String,
    pub sid: i64,
    pub role: i8, // 0-普通成员, 1-管理员
    pub jointime: NaiveDateTime,
}