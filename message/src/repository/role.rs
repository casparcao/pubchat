use anyhow::Result;
use sqlx::{MySql, QueryBuilder};

use crate::common::request::Page;
use crate::model::role::Role;
use crate::repository::db;
use crate::vo::role::RoleListRequest;

pub async fn select_roles(page: Page, param: RoleListRequest) -> Result<(Vec<Role>, i64)> {
    let mut connection = db::connection().await?;
    let mut builder: QueryBuilder<MySql> = QueryBuilder::new("SELECT * FROM role where 1 = 1 ");
    build_sql(& param, &mut builder);
    if let Some(fd) = &page.fd {
        builder
        .push(" ORDER BY ")
        .push(fd)
        .push(" ")
        .push(&page.dir())
        .push(" ");
    }
    builder.push(" LIMIT ")
        .push_bind(page.ps)
        .push(" OFFSET ")
        .push_bind(page.offset());
    let user:Vec<Role> = builder
        .build_query_as()
        .fetch_all(connection.as_mut())
        .await?;
    let mut builder: QueryBuilder<MySql> = QueryBuilder::new("SELECT COUNT(1) AS count FROM role where 1 = 1 ");
    build_sql(&param, &mut builder);
    let count:(i64,) = builder
        .build_query_as()
        .fetch_one(connection.as_mut())
        .await?;
    Ok((user, count.0))
}

fn build_sql(param: &RoleListRequest, builder: &mut QueryBuilder<'static, MySql>) {
    if let Some(name) = &param.name {
        builder.push(" AND name LIKE ").push_bind(format!("%{}%", name));
    }
    if let Some(code) = &param.code {
        builder.push(" AND code LIKE ").push_bind(format!("%{}%", code));
    }
}

pub async fn insert_role(role: Role) -> Result<()>{
    let mut connection = db::connection().await?;
    sqlx::query("INSERT INTO role (id, name, code, builtin, oid, creator) VALUES (?, ?, ?, ?, ?, ?)")
        .bind(role.id)
        .bind(role.name)
        .bind(role.code)
        .bind(role.builtin)
        .bind(role.oid)
        .bind(role.creator)
        .execute(connection.as_mut())
        .await?;
    Ok(())
}

pub async fn delete_role(id: i64) -> Result<()>{
    let mut connection = db::connection().await?;
    sqlx::query("DELETE FROM role WHERE id = ?")
        .bind(id)
        .execute(connection.as_mut())
        .await?;
    Ok(())
}