use std::path::Path;

use core::extract::Multipart;
use anyhow::Result;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use core::auth::User;
use core::response::ApiErr;

#[cfg(target_os="linux")]
const STORE_PATH: &str = "/home/data/images/";
#[cfg(target_os="windows")]
const STORE_PATH: &str = "D:/tmp/";

//该路径对应nginx中location /images {}
const ACCESS_PATH: &str = "/images/";

pub async fn upload_avatar(claims: User, mut multipart: Multipart) -> Result<String>{
    if let Some(field) = multipart.inner.next_field().await? {
        let name = field.file_name();
        if name.is_none(){
            return Err(ApiErr::Bad(400, "文件名缺失".to_string()).into());
        }
        let name = name.unwrap();
        let path = Path::new(name);
        let ext = path.extension();
        if ext.is_none(){
            return Err(ApiErr::Bad(400, "无法确认文件类型".to_string()).into());
        }
        let ext = ext.unwrap();
        let name = format!("{}.{}", snowflaker::next_id()?, ext.to_string_lossy());
        let path = format!("{}{}", STORE_PATH, name);
        let data = field.bytes().await?;
        let mut file = File::create(&path).await?;
        file.write_all(data.as_ref()).await?;
        file.flush().await?;
        log::info!("Length of `{}` is {} bytes", path, data.len());
        let path = format!("{}{}", ACCESS_PATH, name);
        crate::repository::user::update_avatar(claims.id, &path).await?;
        return Ok(path);
    }
    Err(ApiErr::Bad(400, "文件信息缺失".to_string()).into())
}

pub(crate) async fn set_username(user: User, username: String) -> Result<()>{
    crate::repository::user::update_username(user.id, username).await
}

pub(crate) async fn select_user(user: User) -> Result<crate::model::user::User>{
    let result = crate::repository::user::select_user_by_id(user.id).await?;
    result.ok_or(ApiErr::Bad(404, "无该用户".to_string()).into())
}
