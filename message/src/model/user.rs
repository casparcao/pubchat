use chrono::NaiveDateTime;
use sqlx::FromRow;
use crate::common::enums::Gender;

#[derive(FromRow)]
pub struct User{
    pub id: i64,
    pub name: String,
    pub password: String,
    pub gender: Gender,
    pub age: i8,
    pub createtime: NaiveDateTime,
    pub open_id: Option<String>,
    pub union_id: Option<String>,
    pub avatar: Option<String>,
}

impl Default for User {
    fn default() -> Self {
        Self { id: Default::default(), 
            name: Default::default(), 
            password: Default::default(), 
            gender: Gender::U,
            age: Default::default(), 
            createtime: chrono::Utc::now().naive_utc(),
            open_id: None,
            union_id: None,
            avatar: None,
        }
    }
}