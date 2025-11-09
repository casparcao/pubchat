use core::request::Page;
use core::response::ApiResponse;
use core::response::ApiErr;

use anyhow::Result;
use axum::routing::get;
use axum::Router;
use core::extract::Path;
use core::extract::Query;
use crate::service::message;
use core::api::types::message::Message;

pub async fn get_messages_by_session(
    Path(session): Path<i64>,
    Query(params): Query<Page>,
) -> Result<ApiResponse<Message>, ApiErr>{
    let limit = params.ps;
    Ok(ApiResponse::List(message::get_messages_by_session(session, limit).await?, 0))
}

pub async fn index() -> Result<ApiResponse<Message>, ApiErr> {
    Ok(ApiResponse::List(vec![], 0))
}

pub fn router() -> Router {
    Router::new()
        .route("/{session}/messages", get(get_messages_by_session))
}
