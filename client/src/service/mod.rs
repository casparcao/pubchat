use std::sync::OnceLock;

pub mod connection;
pub mod friend;
pub mod login;
pub mod message;
pub mod session;

pub static CONNECTION_HOST : OnceLock<String> = OnceLock::new();
pub static USER_HOST : OnceLock<String> = OnceLock::new();
pub static SESSION_HOST: OnceLock<String> = OnceLock::new();

pub(crate) fn init() {
    let user_host = dotenv::var("USER_SERVER_URL").expect("请设置USER_SERVER_URL环境变量");
    let connection_host = dotenv::var("CONNECTION_SERVER_URL").expect("请设置CONNECTION_SERVER_URL环境变j'j");
    let session_host = dotenv::var("SESSION_SERVER_URL").expect("请设置SESSION_SERVER_URL环境变j'j");
    USER_HOST.set(user_host).expect("初始化USER_HOST失败");
    CONNECTION_HOST.set(connection_host).expect("初始化CONNECTION_HOST失败");
    SESSION_HOST.set(session_host).expect("初始化SESSION_HOST失败");
}

pub(crate) fn user_host() -> String {
    USER_HOST.get().expect("USER_HOST未初始化").clone()
}

pub(crate) fn connection_host() -> String {
    CONNECTION_HOST.get().expect("CONNECTION_HOST未初始化").clone()
}

pub(crate) fn session_host() -> String {
    SESSION_HOST.get().expect("SESSION_HOST未初始化").clone()
}