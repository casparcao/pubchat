use core::request::Page;
use core::response::ApiResponse;
use core::response::ApiErr;

use anyhow::Result;
use axum::routing::get;
use axum::Router;
use core::extract::Path;
use core::extract::Query;
use crate::service::message;
use crate::model::message::Message;

pub async fn get_messages_by_room(
    Path(room_id): Path<i64>,
    Query(params): Query<Page>,
) -> Result<ApiResponse<Message>, ApiErr>{
    let limit = params.ps;
    Ok(ApiResponse::List(message::get_messages_by_room(room_id, limit).await?, 0))
}

pub async fn get_messages_by_speaker(
    Path(speaker_id): Path<i64>,
    Query(params): Query<Page>,
) -> Result<ApiResponse<Message>, ApiErr> {
    let limit = params.ps;
    
    Ok(ApiResponse::List(message::get_messages_by_speaker(speaker_id, limit).await?, 0))
}

pub async fn index() -> Result<ApiResponse<Message>, ApiErr> {
    Ok(ApiResponse::List(vec![], 0))
}

pub fn router() -> Router {
    Router::new()
        .route("/messages/{room_id}", get(get_messages_by_room))
}
