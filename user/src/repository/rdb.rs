use std::sync::OnceLock;

use anyhow::Result;
use dotenv;

use crate::common::response::ApiErr;

pub static POOL: OnceLock<redis::Client> = OnceLock::new();

pub async fn init(){
    let connstring = dotenv::var("REDIS_URL").expect("请设置REDIS连接地址REDIS_URL");
    let client = redis::Client::open(connstring).expect("连接redis失败......");
    POOL.set(client).expect("保存Redis客户端失败")
}

async fn get() -> Result<&'static redis::Client> {
    POOL.get().ok_or(ApiErr::Error("获取Redis客户端失败").into())
}

pub async fn connection() -> Result<redis::Connection>{
    Ok(get().await?.get_connection()?)
}