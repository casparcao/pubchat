use tower::ServiceBuilder;
use tower_http::{cors::{Any, CorsLayer}, validate_request::ValidateRequestHeaderLayer};
use crate::common::auth;
use axum::{error_handling::HandleErrorLayer, routing::any, BoxError, Router};
use crate::common::response::ApiErr;

pub fn init() -> Result<Router, String>{
    let app = Router::new();
    Ok(app.route("/index", axum::routing::get(crate::controller::index::index))
        .merge(role_route())
        .merge(food_route())
        .merge(user_route())
        .merge(contact_route())
        .merge(crate::controller::ratings::router())
        .layer(ValidateRequestHeaderLayer::custom(auth::AuthHeader{}))
        .merge(crate::controller::brands::router())
        .route("/login", axum::routing::post(crate::controller::auth::login))
        .route("/signup", axum::routing::post(crate::controller::auth::signup))
        .fallback(any(not_found))
        .layer(CorsLayer::new().allow_methods(Any).allow_origin(Any))
        .layer(ServiceBuilder::new()
                .layer(HandleErrorLayer::new(error_handling))
                .timeout(std::time::Duration::from_secs(3600))))
}

fn user_route() -> Router{
    Router::new()
        .route("/user/avatar", axum::routing::post(crate::controller::user::upload_avatar))
        .route("/user/username", axum::routing::post(crate::controller::user::set_username))
        .route("/user/current", axum::routing::get(crate::controller::user::select_current_user))
}

fn contact_route() -> Router{
    Router::new()
        .route("/contacts", axum::routing::get(crate::controller::contact::get_list))
        .route("/contacts/{there}", axum::routing::post(crate::controller::contact::add))
        .route("/contacts/{there}", axum::routing::delete(crate::controller::contact::remove))
}

fn food_route() -> Router{
    Router::new()
        .route("/food/choice", axum::routing::get(crate::controller::food::do_choice))
        .route("/food/decision", axum::routing::post(crate::controller::food::do_decision))
        .route("/food/populars", axum::routing::get(crate::controller::food::select_popular_foods))
        .route("/food/list", axum::routing::get(crate::controller::food::select_foods))
        .route("/food/chosen", axum::routing::get(crate::controller::food::select_chosen))
        .route("/food/tags", axum::routing::get(crate::controller::food::select_tags))
}

fn role_route() -> Router{
    Router::new()
        .route("/roles", axum::routing::get(crate::controller::role::select_roles))
        .route("/roles", axum::routing::post(crate::controller::role::create_role))
        .route("/roles/{id}", axum::routing::delete(crate::controller::role::delete_role))
}

pub async fn not_found() -> ApiErr{
    ApiErr::Bad(404, "未找到指定资源...")
}

async fn error_handling(err: BoxError) -> ApiErr {
    ApiErr::Bad(500, "服务器内部错误")
}