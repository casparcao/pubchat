use axum::{
    routing::get,
    Router,
};
use crate::common::extract::Path;
use crate::common::response::ApiErr;
use crate::{common::response::ApiResponse, model::brand::*};
use crate::service::brand;

async fn get_brands_by_category(
    Path(category): Path<String>,
) -> Result<ApiResponse<Brand>, ApiErr>{
    Ok(ApiResponse::List(brand::get_brands_by_category(&category).await?, 0))
}

async fn get_top_brands() -> Result<ApiResponse<Brand>, ApiErr> {
    Ok(ApiResponse::List(brand::get_top_brands().await?, 0))
}

pub fn router() -> Router {
    Router::new()
        .route("/brands/category/{category}", get(get_brands_by_category))
        .route("/brands/top", get(get_top_brands))
}