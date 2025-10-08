use chrono::NaiveDateTime;
use sqlx::FromRow;
///组织信息
#[derive(FromRow)]
pub struct Org{
    pub id: i64,
    pub name: String,
    pub code: String,
    pub createtime: NaiveDateTime
}

#[derive(FromRow)]
pub struct OrgUserRelation{
   pub id: i64,
   pub oid: i64,
   pub uid: i64 
}