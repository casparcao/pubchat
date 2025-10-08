use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Rating {
    pub id: i64,
    pub user_id: i64,
    pub model_id: i64,
    pub rating: i32,
    pub review_text: Option<String>,
    pub purchase_verified: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateRating {
    pub model_id: i64,
    pub rating: i32,
    pub review_text: Option<String>,
    pub purchase_verified: bool,
}

#[derive(Debug, Serialize)]
pub struct RatingStats {
    pub average_rating: f64,
    pub total_ratings: i64,
    pub rating_distribution: Vec<(i32, i64)>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct PlainRatingStats {
    pub average_rating: f64,
    pub total_ratings: i64,
    pub ratings: String,
}