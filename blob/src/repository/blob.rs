use anyhow::Result;
use crate::model::blob::Blob;
use crate::repository::db;


pub async fn create_blob(req: Blob) -> Result<()> {
    let pool = db::get().await;
    
    let blob = sqlx::query_as!(
        Blob,
        r#"
        INSERT INTO blobs (
            id, name, path, size, type, provider, bucket, open, exp, createtime, uid, hash
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12
        )
        RETURNING 
            id, name, path, size, type as mime_type, provider, bucket, open, exp, 
            createtime, uid, hash, deleted
        "#,
        id,
        req.name,
        req.path,
        req.size,
        req.mime_type,
        req.provider,
        req.bucket,
        req.open,
        req.exp,
        now,
        req.uid,
        req.hash
    )
    .fetch_one(pool)
    .await?;

    Ok(())
}

pub async fn get_blob_by_id(id: i64) -> Result<Option<Blob>> {
    let pool = db::pool().await;
    
    let blob = sqlx::query_as!(
        Blob,
        r#"
        SELECT 
            id, name, path, size, type as mime_type, provider, bucket, open, exp, 
            createtime, uid, hash, deleted
        FROM blobs 
        WHERE id = $1 AND deleted = false
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(blob)
}