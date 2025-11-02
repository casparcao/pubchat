use anyhow::Result;
use sqlx::{MySql, QueryBuilder};
use crate::{model::food::{Food, Tag}, repository::db};
use core::request::Page;
use crate::model::food::{UserFoodChoice, UserFoodChoiceDetail};
use crate::vo::food::{ChosenRequest, FoodRequest};

pub async fn count_by_tag(tid: i64) -> Result<i32> {
    let mut connection = db::connection().await?;
    let count: (i32,)
        = sqlx::query_as("SELECT count(id) FROM food_tag WHERE tid = ?")
        .bind(tid)
        .fetch_one(connection.as_mut())
        .await?;
    Ok(count.0)
}

pub async fn count() -> Result<i32> {
    let mut connection = db::connection().await?;
    let count: (i32,)
        = sqlx::query_as("SELECT count(id) FROM food")
        .fetch_one(connection.as_mut())
        .await?;
    Ok(count.0)
}

pub async fn select_one_by_tag(tid: i64, offset: i32) -> Result<Option<Food>> {
    let mut connection = db::connection().await?;
    let result: Option<Food>
        = sqlx::query_as("select f.id, f.name, f.pick_count
                                from food_tag ft
                                left join food f on ft.fid = f.id
                                where ft.tid = ?
                                limit ?, 1")
        .bind(tid)
        .bind(offset)
        .fetch_optional(connection.as_mut())
        .await?;
    Ok(result)
}

pub async fn select_one(offset: i32) -> Result<Option<Food>> {
    let mut connection = db::connection().await?;
    let result: Option<Food>
        = sqlx::query_as("select id, name, pick_count
                                from food
                                limit ?, 1")
        .bind(offset)
        .fetch_optional(connection.as_mut())
        .await?;
    Ok(result)
}

pub async fn select_tags(fid: i64) -> Result<Vec<Tag>> {
    let mut connection = db::connection().await?;
    let result: Vec<Tag>
        = sqlx::query_as("select t.id, t.name
                                from food_tag ft
                                left join tag t on ft.tid = t.id
                                where ft.fid = ?")
        .bind(fid)
        .fetch_all(connection.as_mut())
        .await?;
    Ok(result)
}

pub(crate) async fn select_populars() -> Result<Vec<Food>>{
    let mut connection = db::connection().await?;
    let result: Vec<Food>
        = sqlx::query_as("select id, name, pick_count
                         from food
                         limit 20")
        .fetch_all(connection.as_mut())
        .await?;
    Ok(result)
}

pub(crate) async fn select_page(page: Page, param: FoodRequest) -> Result<(Vec<Food>, i64)>{
    let mut connection = db::connection().await?;
    let mut builder: QueryBuilder<MySql> = QueryBuilder::new("SELECT * FROM food where 1 = 1 ");
    build_sql(& param, &mut builder);
    builder
        .push(" ORDER BY pick_count DESC")
        .push(" LIMIT ")
        .push_bind(page.ps)
        .push(" OFFSET ")
        .push_bind(page.offset());
    let user:Vec<Food> = builder
        .build_query_as()
        .fetch_all(connection.as_mut())
        .await?;
    let mut builder: QueryBuilder<MySql> = QueryBuilder::new("SELECT COUNT(1) AS count FROM food where 1 = 1 ");
    build_sql(&param, &mut builder);
    let count:(i64,) = builder
        .build_query_as()
        .fetch_one(connection.as_mut())
        .await?;
    Ok((user, count.0))
}

fn build_sql(param: &FoodRequest, builder: &mut QueryBuilder<'static, MySql>) {
    if let Some(name) = &param.name {
        builder.push(" AND name LIKE ").push_bind(format!("%{}%", name));
    }
}

pub(crate) async fn insert_choice(row: UserFoodChoice) -> Result<()>{
    let mut connection = db::connection().await?;
    sqlx::query("INSERT INTO user_food_choice (id, uid, fid) VALUES (?, ?, ?)")
        .bind(row.id)
        .bind(row.uid)
        .bind(row.fid)
        .execute(connection.as_mut())
        .await?;
    Ok(())
}

pub(crate) async fn incr_pick_count(id: i64) -> Result<()>{
    let mut connection = db::connection().await?;
    sqlx::query("UPDATE food SET pick_count = pick_count + 1 WHERE id = ?")
        .bind(id)
        .execute(connection.as_mut())
        .await?;
    Ok(())
}

pub(crate) async fn select_chosen(uid: i64, page: Page, param: ChosenRequest) -> Result<(Vec<UserFoodChoiceDetail>, i64)>{
    let mut connection = db::connection().await?;
    let mut builder: QueryBuilder<MySql> = QueryBuilder::new(
        "SELECT f.id as id, f.name as name, ufc.createtime as time
            FROM user_food_choice ufc
            LEFT JOIN food f on ufc.fid = f.id
            where ufc.uid = ");
    builder.push_bind(uid);
    if let Some(name) = &param.name {
        builder.push(" AND f.name LIKE ").push_bind(format!("%{}%", name));
    }
    builder
        .push(" ORDER BY ufc.createtime DESC")
        .push(" LIMIT ")
        .push_bind(page.ps)
        .push(" OFFSET ")
        .push_bind(page.offset());
    let user:Vec<UserFoodChoiceDetail> = builder
        .build_query_as()
        .fetch_all(connection.as_mut())
        .await?;
    let mut builder: QueryBuilder<MySql> = QueryBuilder::new(
        "SELECT COUNT(1)
            FROM user_food_choice ufc
            LEFT JOIN food f on ufc.fid = f.id
            where ufc.uid = ");
    builder.push_bind(uid);
    if let Some(name) = &param.name {
        builder.push(" AND f.name LIKE ").push_bind(format!("%{}%", name));
    }
    let count:(i64,) = builder
        .build_query_as()
        .fetch_one(connection.as_mut())
        .await?;
    Ok((user, count.0))
}