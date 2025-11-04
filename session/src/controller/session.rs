use core::auth::User;
use core::extract::Json;
use core::extract::Path;
use core::response::ApiResponse;
use core::response::ApiErr;
use anyhow::Result;
use axum::Extension;
use axum::routing::{get, post};
use axum::Router;
use crate::service::session;
use crate::model::session::Session;
use crate::vo::session::SessionDetailResponse;
use crate::vo::session::{CreateSessionRequest};

pub async fn create_session(
    Extension(claims): Extension<User>,
    Json(payload): Json<CreateSessionRequest>,
) -> Result<ApiResponse<Session>, ApiErr> {
    let session = session::create_session(claims.id, payload).await?;
    Ok(ApiResponse::One(session))
}

// 获取当前用户的会话列表
pub async fn get_current_user_sessions(
    Extension(claims): Extension<User>,
) -> Result<ApiResponse<Session>, ApiErr> {
    let sessions = session::get_sessions_by_user(claims.id).await?;
    Ok(ApiResponse::List(sessions, 0))
}

pub async fn get_session_details(
    Path(id): Path<i64>,
) -> Result<ApiResponse<SessionDetailResponse>, ApiErr> {
    // 获取会话基本信息
    let detail = session::get_session_by_id(id).await?;
    Ok(ApiResponse::One(detail))
}

pub fn router() -> Router {
    Router::new()
        .route("/sessions", post(create_session))
        .route("/user/sessions", get(get_current_user_sessions))
        .route("/sessions/{id}", get(get_session_details))
}