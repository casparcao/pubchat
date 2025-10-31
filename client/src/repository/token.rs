use core::auth::Token;
use std::fs;
use std::path::Path;
use anyhow::Result;
use log::info;

/// 保存token到本地文件
pub fn save_token(token: &str, exp: u128) -> Result<()> {
    let stored_token = Token {
        token: token.to_string(),
        exp,
    };
    
    let json = serde_json::to_string(&stored_token)?;
    fs::write(get_token_file_path()?, json)?;
    Ok(())
}

/// 从本地文件读取token
pub fn load_token() -> Result<Option<Token>> {
    let token_path = get_token_file_path()?;
    info!("Loading token from {}", token_path);
    if !Path::new(&token_path).exists() {
        return Ok(None);
    }
    let content = fs::read_to_string(token_path)?;
    info!("Token loaded: {:?}", content);
    let stored_token: Token = serde_json::from_str(&content)?;
    Ok(Some(stored_token))
}

/// 删除本地存储的token文件
pub fn clear_token() -> Result<()> {
    let token_path = get_token_file_path()?;
    
    if Path::new(&token_path).exists() {
        fs::remove_file(token_path)?;
    }
    Ok(())
}

/// 检查token是否仍然有效
pub fn is_token_valid(stored_token: &Token) -> bool {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();
    
    stored_token.exp > now
}

/// 获取token文件路径
fn get_token_file_path() -> Result<String> {
    let home_dir = std::env::home_dir().unwrap();
    let token_path = format!("{}/.pubchat.token", home_dir.display());
    Ok(token_path)
}