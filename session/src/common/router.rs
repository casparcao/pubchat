use tower::ServiceBuilder;
use tower_http::{cors::{Any, CorsLayer}, validate_request::ValidateRequestHeaderLayer};
use anyhow::Result;
use axum::{error_handling::HandleErrorLayer, routing::any, BoxError, Router};
use core::response::ApiErr;
use crate::controller::message;

pub fn init() -> Result<Router> {
    let app = Router::new();
    Ok(app.route("/index", axum::routing::get(crate::controller::message::index))
        .merge(message::router())
        .layer(ValidateRequestHeaderLayer::custom(core::auth::AuthHeader{}))
        .fallback(any(not_found))
        .layer(CorsLayer::new().allow_methods(Any).allow_origin(Any))
        .layer(ServiceBuilder::new()
                .layer(HandleErrorLayer::new(error_handling))
                .timeout(std::time::Duration::from_secs(3600))))
}


pub async fn not_found() -> ApiErr{
    ApiErr::Bad(404, "未找到指定资源...".to_string())
}

async fn error_handling(err: BoxError) -> ApiErr {
    ApiErr::Bad(500, "服务器内部错误".to_string())
}