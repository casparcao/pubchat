use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct Friend {
    pub id: i64,
    pub user_id: i64,
    pub friend_id: i64,
    pub status: i8, // 1-正常, 2-拉黑
    pub createtime: NaiveDateTime,
    pub updatetime: NaiveDateTime,
}

impl Default for Friend {
    fn default() -> Self {
        Self { 
            id: 0,
            user_id: 0,
            friend_id: 0,
            status: 1,
            createtime: chrono::Utc::now().naive_utc(),
            updatetime: chrono::Utc::now().naive_utc(),
        }
    }
}