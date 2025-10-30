use tower::ServiceBuilder;
use tower_http::{cors::{Any, CorsLayer}, validate_request::ValidateRequestHeaderLayer};
use anyhow::Result;
use axum::{error_handling::HandleErrorLayer, routing::any, Router};
use core::response::{e404, e500};
use crate::controller::message;
use crate::controller::session;

pub fn init() -> Result<Router> {
    let app = Router::new();
    Ok(app.route("/index", axum::routing::get(crate::controller::message::index))
        .merge(message::router())
        .merge(session::router())
        .layer(ValidateRequestHeaderLayer::custom(core::auth::AuthHeader{}))
        .fallback(any(e404))
        .layer(CorsLayer::new().allow_methods(Any).allow_origin(Any))
        .layer(ServiceBuilder::new()
                .layer(HandleErrorLayer::new(e500))
                .timeout(std::time::Duration::from_secs(3600))))
}

