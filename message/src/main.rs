use dotenv::dotenv;
use tokio::net::TcpListener;

pub mod model;
pub mod service;
pub mod repository;
pub mod controller;
pub mod vo;
pub mod common;
pub mod test;

use crate::repository::db;
use crate::common::log;
use crate::common::auth;
use crate::common::router;
use crate::repository::rdb;


#[tokio::main]
async fn main() -> Result<(), String> {
    dotenv().ok();
    rdb::init().await;
    db::init().await;
    log::init();
    auth::init();
    let app = router::init().expect("路由模块初始化失败");
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
