use crate::model::rating::{CreateRating, Rating, RatingStats, PlainRatingStats};
use anyhow::Result;
use crate::repository::db;

pub async fn create_rating(
    user_id: i64,
    input: CreateRating,
) -> Result<()> {
    let mut pool = db::connection().await?;
    sqlx::query(
        r#"
        INSERT INTO user_ratings (user_id, model_id, rating, review_text, purchase_verified)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#)
    .bind(user_id)
    .bind(input.model_id)
    .bind(input.rating)
    .bind(input.review_text)
    .bind(input.purchase_verified)
    .execute(pool.as_mut())
    .await?;
    Ok(())
}

pub async fn get_model_ratings(
    model_id: i64,
) -> Result<Vec<Rating>> {
    let mut pool = db::connection().await?;
    let result : Vec<Rating> = sqlx::query_as(
        r#"
        SELECT * FROM user_ratings
        WHERE model_id = ?
        ORDER BY created_at DESC
        "#
    )
    .bind(model_id)
    .fetch_all(pool.as_mut())
    .await?;
    Ok(result)
}

pub async fn get_rating_stats(
    model_id: i64,
) -> Result<RatingStats> {
    let mut pool = db::connection().await?;
    let stats: PlainRatingStats = sqlx::query_as(
        r#"
        SELECT 
            AVG(rating::float8) as average_rating,
            COUNT(*) as total_ratings,
            GROUP_CONCAT(rating) as ratings
        FROM user_ratings
        WHERE model_id = ?
        "#
    )
    .bind(model_id)
    .fetch_one(pool.as_mut())
    .await?;

    let ratings:Vec<&str> = stats.ratings.split(",").collect();
    let mut dist = vec![(1, 0i64), (2, 0), (3, 0), (4, 0), (5, 0)];
    for rating in ratings {
        if let Some(idx) = (rating.parse::<usize>()?).checked_sub(1) {
            if idx < 5 {
                dist[idx].1 += 1;
            }
        }
    }

    Ok(RatingStats {
        average_rating: stats.average_rating,
        total_ratings: stats.total_ratings,
        rating_distribution: dist,
    })
}