use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct Food{
    pub id: i64,
    pub name: String,
    pub pick_count: i32
}

#[derive(FromRow)]
pub struct Tag {
    pub id: i64,
    pub name: String
}

#[derive(FromRow)]
pub struct FoodTag{
    pub id: i64,
    pub tid: i64,
    pub fid: i64
}

#[derive(FromRow)]
pub struct UserFoodChoice{
    pub id: i64,
    pub uid: i64,
    pub fid: i64
}

#[derive(FromRow)]
pub struct UserFoodChoiceDetail{
    pub id: i64,
    pub name: String,
    pub time: NaiveDateTime,
}