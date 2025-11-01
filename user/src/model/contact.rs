use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct Contact {
    pub id: i64,
    pub here: i64,
    pub there: i64,
    pub status: i8, // 1-正常, 2-拉黑
    pub createtime: NaiveDateTime,
    pub updatetime: NaiveDateTime,
}

impl Default for Contact {
    fn default() -> Self {
        Self { 
            id: 0,
            here: 0,
            there: 0,
            status: 1,
            createtime: chrono::Utc::now().naive_utc(),
            updatetime: chrono::Utc::now().naive_utc(),
        }
    }
}