use dotenv::dotenv;
use tokio::net::TcpListener;

pub mod model;
pub mod service;
pub mod repository;
pub mod controller;
pub mod vo;
pub mod common;
pub mod consumer;
pub mod test;

use crate::repository::{db, rdb};
use crate::common::router;


#[tokio::main]
async fn main() -> Result<(), String> {
    dotenv().ok();
    rdb::init().await;
    db::init().await;
    consumer::init().await;
    // 如果session模块有初始化需求，请取消下面这行注释并实现init方法
    // session::init().await;
    
    let app = router::init().expect("路由模块初始化失败");
    let listener = TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}