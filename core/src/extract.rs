use axum::{body::Body, extract::{rejection::{JsonRejection, PathRejection, QueryRejection}, FromRequest, FromRequestParts}, http::StatusCode, response::IntoResponse};
use http::Request;
use axum_extra::extract::multipart::MultipartRejection;

use super::response::ApiErr;

// create an extractor that internally uses `axum::Json` but has a custom rejection
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(ApiRejection))]
pub struct Json<T>(pub T);

#[cfg_attr(docsrs, doc(cfg(feature = "multipart")))]
#[derive(Debug)]
pub struct Multipart{
    pub inner: axum_extra::extract::Multipart
}

impl<S> FromRequest<S> for Multipart
    where
        S: Send + Sync,
{
    type Rejection = ApiRejection;

    async fn from_request(req: Request<Body>, _state: &S) -> Result<Self, Self::Rejection> {
        let multipart = axum_extra::extract::Multipart::from_request(req, _state).await?;
        Ok(Self { inner: multipart })
    }
}

#[derive(FromRequestParts)]
#[from_request(via(axum::extract::Query), rejection(ApiRejection))]
pub struct Query<T>(pub T);

#[derive(FromRequestParts)]
#[from_request(via(axum::extract::Path), rejection(ApiRejection))]
pub struct Path<T>(pub T);

// We create our own rejection type
#[derive(Debug)]
pub struct ApiRejection {
    status: StatusCode,
    message: String,
}

// We implement `From<JsonRejection> for ApiError`
impl From<JsonRejection> for ApiRejection {
    fn from(rejection: JsonRejection) -> Self {
        Self {
            status: rejection.status(),
            message: rejection.body_text(),
        }
    }
}

impl From<QueryRejection> for ApiRejection {
    fn from(rejection: QueryRejection) -> Self {
        Self {
            status: rejection.status(),
            message: rejection.body_text(),
        }
    }
}

impl From<PathRejection> for ApiRejection {
    fn from(rejection: PathRejection) -> Self {
        Self {
            status: rejection.status(),
            message: rejection.body_text(),
        }
    }
}

impl From<MultipartRejection> for ApiRejection{
    fn from(rejection: MultipartRejection) -> Self {
        Self{
            status: rejection.status(),
            message: rejection.body_text(),
        }
    }
}

// We implement `IntoResponse` so `ApiError` can be used as a response
impl IntoResponse for ApiRejection {
    fn into_response(self) -> axum::response::Response {
        log::info!("ApiRejection:{}:{}", self.status, self.message);
        ApiErr::Bad(400, "请求参数不合法，请检查后重新提交").into_response()
    }
}