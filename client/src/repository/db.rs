use std::sync::OnceLock;

use anyhow::{Ok, Result};
use sqlx::{Sqlite, SqlitePool, migrate, pool::PoolConnection};

use core::response::ApiErr;

pub static POOL: OnceLock<SqlitePool> = OnceLock::new();

pub fn init(){
    let path = std::env::home_dir().unwrap().join(".pubchat.sqlite3");
    if !path.exists() {
        std::fs::File::create(&path).expect("创建数据库文件失败......");
    }
    let connstring = path.display().to_string();
    log::error!("{}", &connstring);
    let rt = tokio::runtime::Runtime::new().expect("创建tokio运行时失败......");
    let pool = rt.block_on(SqlitePool::connect(&format!("sqlite:{}", &connstring))).expect("连接数据库失败......");
    rt.block_on(migrate!("./migrations").run(&pool)).expect("迁移数据库失败......");
    POOL.set(pool).expect("数据库连接池初始化失败");
}

async fn get() -> Result<&'static SqlitePool> {
    POOL.get().ok_or(ApiErr::Error("获取Redis客户端失败".to_string()).into())
}


pub async fn connection() -> Result<PoolConnection<Sqlite>>{
    Ok(get().await?.acquire().await?)
}