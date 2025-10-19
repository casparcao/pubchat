use anyhow::Result;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use crate::service::message::MessageService;
use crate::model::message::Message;

#[derive(Debug, Deserialize)]
pub struct MessageQuery {
    limit: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    messages: Vec<Message>,
}

pub async fn get_messages_by_room(
    State(service): State<MessageService>,
    Path(room_id): Path<i64>,
    Query(params): Query<MessageQuery>,
) -> Result<Json<MessageResponse>, (axum::http::StatusCode, String)> {
    let limit = params.limit.unwrap_or(50);
    
    match service.get_messages_by_room(room_id, limit).await {
        Ok(messages) => Ok(Json(MessageResponse { messages })),
        Err(e) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn get_messages_by_speaker(
    State(service): State<MessageService>,
    Path(speaker_id): Path<i64>,
    Query(params): Query<MessageQuery>,
) -> Result<Json<MessageResponse>, (axum::http::StatusCode, String)> {
    let limit = params.limit.unwrap_or(50);
    
    match service.get_messages_by_speaker(speaker_id, limit).await {
        Ok(messages) => Ok(Json(MessageResponse { messages })),
        Err(e) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}