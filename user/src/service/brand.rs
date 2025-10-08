use crate::model::brand::Brand;
use crate::repository::brand;
use anyhow::Result;


pub async fn get_top_brands() -> Result<Vec<Brand>> {
    brand::get_top_brands().await
}

pub async fn get_brands_by_category(category: &str) -> Result<Vec<Brand>> {
    brand::get_brands_by_category(category).await
}