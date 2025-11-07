use std::{sync::OnceLock, time::Duration};

use anyhow::{Ok, Result};
use dotenv;
use sqlx::{migrate, MySql, mysql::MySqlPoolOptions, MySqlPool, pool::PoolConnection};

use core::response::ApiErr;

pub static POOL: OnceLock<MySqlPool> = OnceLock::new();

pub async fn init(){
    let connstring = dotenv::var("DATABASE_URL").expect("请设置数据库连接地址DATABASE_URL");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(30))
        .acquire_slow_threshold(Duration::from_secs(3))
        .connect(&connstring)
        .await
        .expect("连接数据库失败......");
    migrate!("./migrations").run(&pool).await.expect("迁移数据库失败......");
    POOL.set(pool).expect("数据库连接池初始化失败");
}

pub(crate) async fn get() -> Result<&'static MySqlPool> {
    POOL.get().ok_or(ApiErr::Error("获取Redis客户端失败".to_string()).into())
}


pub async fn connection() -> Result<PoolConnection<MySql>>{
    Ok(get().await?.acquire().await?)
}