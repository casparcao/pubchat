use anyhow::Error;
use axum::{response::{IntoResponse, Response}, Json};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub enum ApiResponse<T: Serialize>{
    One(T),
    //分页数据
    List(Vec<T>, i64),
}

#[derive(Debug, Deserialize, Serialize, Error)]
pub enum ApiErr{
    //客户请求不合法
    #[error("{0}")]
    Bad(u16, &'static str),
    //服务器端异常
    #[error("")]
    Error(&'static str)
}

impl From<Error> for ApiErr{
    fn from(value: Error) -> Self {
        // ApiErr::Bad("请求不合法，请检查后重新请求".to_string())
        let api_error = value.is::<ApiErr>();
        if api_error{
            let r = value.downcast::<ApiErr>();
            if r.is_ok(){
                return r.unwrap();
            }
        }else{
            log::error!("异常>>{:?}", value);
        }
        return ApiErr::Error("服务异常，请稍后再试");
    }
}

impl IntoResponse for ApiErr{
    fn into_response(self) -> Response {
        match self {
            Self::Bad(code, msg) =>
                (axum::http::StatusCode::from_u16(code).unwrap(), Json(ApiResult::<String>::bad(msg.to_string()))).into_response(),
            Self::Error(msg) =>
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResult::<String>::bad(msg.to_string()))).into_response(),
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T>{
    fn into_response(self) -> Response {
        match self {
            Self::One(data) => 
                (axum::http::StatusCode::OK, Json(ApiResult::ok(data))).into_response(),
            Self::List(data, count) =>
                (axum::http::StatusCode::OK, Json(ApiResult::<Vec<T>>::page(data, count))).into_response()
        }
    }
}

#[derive(Serialize, Debug, Deserialize)]
struct ApiResult<T: Serialize>{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

impl <T: Serialize> ApiResult<T>{
    pub fn ok(data: T) -> Self{
        Self{
            data: Some(data),
            message: None,
            ok: true,
            count: None,
        }
    }

    pub fn bad(message: String) -> Self{
        Self{
            data: None,
            message: Some(message),
            ok: false,
            count: None,
        }
    }

    pub fn page(data: T, count: i64) -> Self{
        Self{
            data: Some(data),
            message: None,
            ok: true,
            count: Some(count),
        }
    }
}