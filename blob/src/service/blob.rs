use anyhow::Result;
use chrono::Datelike;
use core::extract::Multipart;
use core::response::ApiErr;
use std::path::{Path};
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use crate::model::blob::Blob;
use crate::repository::blob::{create_blob, get_blob_by_id};
use crate::vo::blob::{BlobResponse, BlobUploadResponse};
use core::auth::User;


#[cfg(target_os="linux")]
const STORE_PATH: &str = "/home/data/images/";
#[cfg(target_os="windows")]
const STORE_PATH: &str = "D:/tmp/";

//该路径对应nginx中location /images {}
const ACCESS_PATH: &str = "/images/";

pub async fn upload_file(
    claims: User, mut multipart: Multipart
) -> Result<BlobUploadResponse> {
    if let Some(field) = multipart.inner.next_field().await? {
        let origin_name = field.file_name();
        if origin_name.is_none(){
            return Err(ApiErr::Bad(400, "文件名缺失".to_string()).into());
        }
        let now = chrono::Utc::now();
        let ctype = field.content_type().unwrap_or("unknown").to_string();
        let name = origin_name.unwrap().to_string();
        let id = snowflaker::next_id()?;
        //文件存储目录
        let dir = Path::new(STORE_PATH)
            .join(now.year().to_string())
            .join(now.month().to_string())
            .join(now.day().to_string());
        // Create uploads directory if it doesn't exist
        fs::create_dir_all(&dir).await?;
        //最终文件存储路径
        let abpath = dir.join(id.to_string());
        let data = field.bytes().await?;
        let size = data.len() as i64;
        let mut file = File::create(&abpath).await?;
        file.write_all(data.as_ref()).await?;
        file.flush().await?;
        log::info!("Length of `{}` is {} bytes", &abpath.display(), size);
        // Save blob metadata to database
        let create_req = Blob {
            id: id as i64,
            name: name,
            path: abpath.display().to_string(),
            size,
            btype: ctype,
            provider: "local".to_string(),
            bucket: None,
            open: false,
            exp: Some(now + chrono::Duration::days(7)),
            uid: claims.id,
            hash: None, // In a real implementation, we would calculate a hash
            deleted: false,
            createtime: now,
        };
        create_blob(create_req).await?;
        return Ok(BlobUploadResponse{id: id as i64, url: format!("{}{}", ACCESS_PATH, id)});
    }
    Err(ApiErr::Bad(400, "文件信息缺失".to_string()).into())
}

pub async fn get_blob(id: i64) -> Result<BlobResponse> {
    let opt: Option<Blob> = get_blob_by_id(id).await?;
    if opt.is_none() {
        return Err(ApiErr::Bad(404, "文件不存在".to_string()).into());
    }
    let b = opt.unwrap();
    if b.exp.is_some() && b.exp.unwrap() < chrono::Utc::now() {
        return Err(ApiErr::Bad(410, "文件已过期".to_string()).into());
    }
    Ok(BlobResponse{id: b.id, name: b.name, size: b.size, 
        exp: b.exp.map(|e| format!("{}", e.format("%Y-%m-%d"))),
        //todo 返回访问路径
        path: b.path
    })
}