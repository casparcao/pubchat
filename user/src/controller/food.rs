use axum::Extension;
use core::auth::User;
use core::extract::{Json, Query};
use core::request::Page;
use core::response::{ApiErr, ApiResponse};
use crate::vo::food::{ChoiceRequest, ChoiceResponse, ChosenRequest, ChosenResponse, DecideRequest, FoodRequest, FoodResponse, FoodTagRequest, TagResponse};

//随机选择一个食物，返回给前端
pub async fn do_choice(Query(param): Query<ChoiceRequest>) -> Result<ApiResponse<ChoiceResponse>, ApiErr> {
    let result = crate::service::food::choice(param.tag).await?;
    Ok(ApiResponse::One(result))
}

pub async fn do_decision(Extension(claims): Extension<User>, Json(body): Json<DecideRequest>)
    -> Result<ApiResponse<()>, ApiErr> {
    Ok(ApiResponse::One(crate::service::food::decide(&claims, body).await?))
}

pub async fn select_popular_foods() -> Result<ApiResponse<FoodResponse>, ApiErr>{
    let result = crate::service::food::populars().await?;
    Ok(ApiResponse::List(result, 0))
}

pub async fn select_foods(Query(page): Query<Page>, Query(param): Query<FoodRequest>)
    -> Result<ApiResponse<FoodResponse>, ApiErr> {
    let result = crate::service::food::list(page, param).await?;
    Ok(ApiResponse::List(result.0, result.1))
}

pub async fn select_tags(Query(param): Query<FoodTagRequest>) -> Result<ApiResponse<TagResponse>, ApiErr> {
    let result = crate::service::food::tags(param).await?;
    Ok(ApiResponse::List(result, 0))
}

pub async fn select_chosen(Extension(claims): Extension<User>, Query(page): Query<Page>, Query(param): Query<ChosenRequest>)
    -> Result<ApiResponse<ChosenResponse>, ApiErr>{
    let result = crate::service::food::chosen(claims.id, page, param).await?;
    Ok(ApiResponse::List(result.0, result.1))
}