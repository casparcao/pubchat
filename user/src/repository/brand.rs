use anyhow::Result;
use crate::model::brand::Brand;
use crate::repository::db;


pub async fn get_brands_by_category(category: &str) -> Result<Vec<Brand>> {
    let mut connection = db::connection().await?;
    let result : Vec<Brand> = sqlx::query_as(r#"
        SELECT * FROM brands 
        WHERE category = ? 
        ORDER BY rating DESC
        "#)
    .bind(category)
    .fetch_all(connection.as_mut())
    .await?;
    Ok(result)
}

pub async fn get_top_brands() -> Result<Vec<Brand>> {
    let mut connection = db::connection().await?;
    let result : Vec<Brand> = sqlx::query_as(r#"
        SELECT * FROM brands 
        WHERE is_top_brand = true 
        ORDER BY rating DESC
        "#)
    .fetch_all(connection.as_mut())
    .await?;
    Ok(result)
}
pub async fn get_brand_by_id(id: i64) -> Result<Option<Brand>> {
    let mut connection = db::connection().await?;
    let result: Option<Brand> = sqlx::query_as(r#"
        SELECT * FROM brands WHERE ID = ?
        "#)
    .bind(id)
    .fetch_optional(connection.as_mut())
    .await?;
    Ok(result)
}

pub async fn list_brands() -> Result<Vec<Brand>> {
    let mut connection = db::connection().await?;
    let result: Vec<Brand> = sqlx::query_as(r#"
        SELECT * FROM brands ORDER BY rating DESC
        "#)
    .fetch_all(connection.as_mut())
    .await?;
    Ok(result)
}

pub async fn get_product_by_id(id: i64) -> Result<Option<Brand>> {
    let mut connection = db::connection().await?;
    let result : Option<Brand> = sqlx::query_as(
        r#"
        SELECT b.* FROM brands b
        INNER JOIN product_models p ON p.brand_id = b.id
        WHERE p.id = ?
        "#
    )
    .bind(id)
    .fetch_optional(connection.as_mut())
    .await?;
    Ok(result)
}