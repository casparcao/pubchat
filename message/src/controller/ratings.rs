use axum::{
    Extension,
    routing::{get, post},
    Router,
};
use crate::common::extract::{Json, Path};
use crate::common::{auth::User, response::{ApiErr, ApiResponse}};
use crate::model::rating::{CreateRating, Rating, RatingStats};
use crate::service::rating;

async fn create_rating(
    Extension(claims): Extension<User>,
    Json(input): Json<CreateRating>,
) -> Result<ApiResponse<()>, ApiErr>{
    Ok(ApiResponse::One(rating::create_rating(&claims, input).await?))
}

async fn get_model_ratings(
    Path(model_id): Path<i64>,
) -> Result<ApiResponse<Rating>, ApiErr> {
    Ok(ApiResponse::List(rating::get_model_ratings(model_id).await?, 0))
}

async fn get_rating_stats(
    Path(model_id): Path<i64>,
) -> Result<ApiResponse<RatingStats>, ApiErr> {
    Ok(ApiResponse::One(rating::get_rating_stats(model_id).await?))
}

pub fn router() -> Router {
    Router::new()
        .route("/ratings", post(create_rating))
        .route("/ratings/model/{model_id}", get(get_model_ratings))
        .route("/ratings/stats/{model_id}", get(get_rating_stats))
}
