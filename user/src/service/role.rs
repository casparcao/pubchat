use core::auth::User;
use core::request::Page;
use crate::vo::role::{RoleCreateRequest, RoleListRequest};
use crate::model::role::Role;
use crate::repository::role;
use anyhow::Result;

pub async fn select_roles(page: Page, param: RoleListRequest) -> Result<(Vec<Role>, i64)> {
    role::select_roles(page, param).await
}

pub async fn create_role(role: RoleCreateRequest, creator: &User) -> Result<i64> {
    let id = snowflaker::next_id().unwrap(); 
    let role: Role = Role{id: id as i64, name: role.name, 
        code: role.code,
        builtin: 0,
        oid: creator.oid,
        creator: creator.id,
        createtime: chrono::Utc::now().naive_utc()};
    role::insert_role(role).await.map(|()| id as i64)
}

pub async fn delete_role(id: i64) -> Result<()> {
    role::delete_role(id).await
}