use std::sync::OnceLock;

pub mod login;
pub mod connection;
pub mod friend;
pub mod cache;

pub static CONNECTION_HOST : OnceLock<String> = OnceLock::new();
pub static USER_HOST : OnceLock<String> = OnceLock::new();
pub(crate) async fn init() {
    let user_host = dotenv::var("USER_SERVER_URL").expect("请设置USER_SERVER_URL环境变量");
    let connection_host = dotenv::var("CONNECTION_SERVER_URL").expect("请设置CONNECTION_SERVER_URL环境变j'j");
    USER_HOST.set(user_host).expect("初始化USER_HOST失败");
    CONNECTION_HOST.set(connection_host).expect("初始化CONNECTION_HOST失败");
}

pub(crate) fn user_host() -> String {
    USER_HOST.get().unwrap().to_string()
}

pub(crate) fn connection_host() -> String {
    CONNECTION_HOST.get().unwrap().to_string()
}