use core::auth;

use dotenv::dotenv;
use tokio::net::TcpListener;
use anyhow::Result;

pub mod model;
pub mod service;
pub mod repository;
pub mod controller;
pub mod vo;
pub mod common;
pub mod test;

use crate::repository::{db, rdb};
use crate::common::router;


#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    core::log::init(Some(".pubchat_blob.log"));
    auth::init();
    rdb::init().await;
    db::init().await;
    let app = router::init().expect("路由模块初始化失败");
    let listener = TcpListener::bind("0.0.0.0:3002").await.unwrap();
    log::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}