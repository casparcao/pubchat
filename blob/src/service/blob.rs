use anyhow::Result;
use core::extract::Multipart;
use core::response::ApiErr;
use std::path::Path;
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use crate::model::blob::Blob;
use crate::repository::blob::{CreateBlobRequest, create_blob, get_blob_by_id};
use crate::vo::blob::UploadBlobResponse;
use core::auth::User;

// Directory to store uploaded files locally
const UPLOADS_DIR: &str = "./uploads";


pub async fn upload_file(
    claims: User, mut multipart: Multipart
) -> Result<UploadBlobResponse> {
    if let Some(field) = multipart.inner.next_field().await? {
        let origin_name = field.file_name();
        if origin_name.is_none(){
            return Err(ApiErr::Bad(400, "文件名缺失".to_string()).into());
        }
        let name = origin_name.unwrap();
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
        // Create uploads directory if it doesn't exist
        fs::create_dir_all(UPLOADS_DIR).await?;

        // Generate a unique file path
        let id = snowflaker::next_id()?;
        let file_path = format!("{}/{}", UPLOADS_DIR, id);
        
        // Write file to disk
        let mut file = fs::File::create(&file_path).await?;
        file.write_all(&data).await?;
        
        // Get file size
        let size = data.len() as i64;
        
        // Save blob metadata to database
        let create_req = Blob {
            name: filename.clone(),
            path: file_path.clone(),
            size,
            btype: mime_type.clone(),
            provider: "local".to_string(),
            bucket: None,
            open: false,
            exp: None,
            uid: user.map(|u| u.id),
            hash: None, // In a real implementation, we would calculate a hash
        };
        
        let blob = create_blob(create_req).await?;
        return Ok(path);
    }
    Err(ApiErr::Bad(400, "文件信息缺失".to_string()).into())
    
    
    let url = format!("/blobs/{}/{}", blob.id, filename);
    
    Ok(UploadBlobResponse {
        id: blob.id,
        url,
    })
}

pub async fn get_blob(id: i64) -> Result<Option<Blob>> {
    get_blob_by_id(id).await
}