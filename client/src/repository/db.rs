use std::sync::OnceLock;

use anyhow::{Ok, Result};
use sqlx::{Sqlite, SqlitePool, migrate, pool::PoolConnection};

use core::response::ApiErr;

pub static POOL: OnceLock<SqlitePool> = OnceLock::new();

pub async fn init(){
    let path = std::env::home_dir().unwrap().join("pubchat.sqlite3");
    if !path.exists() {
        std::fs::File::create(&path).expect("创建数据库文件失败......");
    }
    let connstring = path.display().to_string();
    eprintln!("{}", &connstring);
    let pool = SqlitePool::connect(&format!("sqlite:{}", &connstring)).await.expect("连接数据库失败......");
    migrate!("./migrations").run(&pool).await.expect("迁移数据库失败......");
    POOL.set(pool).expect("数据库连接池初始化失败");
}

async fn get() -> Result<&'static SqlitePool> {
    POOL.get().ok_or(ApiErr::Error("获取Redis客户端失败").into())
}


pub async fn connection() -> Result<PoolConnection<Sqlite>>{
    Ok(get().await?.acquire().await?)
}