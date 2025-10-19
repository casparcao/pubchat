use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::FromRow;
///角色信息
#[derive(FromRow, Serialize)]
pub struct Role{
    pub id: i64,
    pub name: String,
    pub code: String,
    ///是否内置角色
    pub builtin: i8,
    ///组织id
    pub oid: i64,
    //创建人
    pub creator: i64,
    #[serde(skip_serializing)]
    pub createtime: NaiveDateTime
}

#[derive(FromRow, Serialize)]
pub struct UserRoleRelation{
   pub id: i64,
   pub rid: i64,
   pub uid: i64 
}

#[derive(FromRow)]
pub struct Module{
    pub id: i64,
    pub name: String,
    pub code: String,
    pub desc: String,
    pub createtime: NaiveDateTime
}

#[derive(FromRow)]
pub struct RoleModuleRelation{
   pub id: i64,
   pub rid: i64,
   pub mid: i64 
}