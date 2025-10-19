use crate::model::rating::*;
use crate::repository::rating;
use anyhow::Result;
use crate::auth::User;



pub async fn create_rating(
    creator: &User,
    input: CreateRating,
) -> Result<()> {
    rating::create_rating(creator.id, input).await
}

pub async fn get_model_ratings(
    model_id: i64,
) -> Result<Vec<Rating>> {
    rating::get_model_ratings(model_id).await
}

pub async fn get_rating_stats(
    model_id: i64,
) -> Result<RatingStats> {
    rating::get_rating_stats(model_id).await
}
