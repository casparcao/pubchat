use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{cors::{Any, CorsLayer}, validate_request::ValidateRequestHeaderLayer};
use anyhow::Result;
use axum::{error_handling::HandleErrorLayer, routing::{any, get, post, delete}, BoxError, Router};
use core::response::ApiErr;
use crate::controller::message;
use crate::repository::{db, rdb};

pub fn init() -> Result<Router> {

    let app = Router::new()
        .route("/messages/room/:room_id", get(message::get_messages_by_room))
        .route("/messages/speaker/:speaker_id", get(message::get_messages_by_speaker));
    Ok(app)
}


pub async fn not_found() -> ApiErr{
    ApiErr::Bad(404, "未找到指定资源...")
}

