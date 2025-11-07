use anyhow::Result;
use crate::model::blob::Blob;
use crate::repository::db;


pub async fn create_blob(req: Blob) -> Result<()> {
    let mut connection = db::connection().await?;
    // 插入会话记录
    sqlx::query(r#"
        INSERT INTO blobs (
            id, name, path, size, btype, provider, bucket, open, exp, createtime, uid, hash, deleted
        ) VALUES (
            ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
        )"#)
        .bind(&req.id)
        .bind(&req.name)
        .bind(&req.path)
        .bind(&req.size)
        .bind(&req.btype)
        .bind(&req.provider)
        .bind(&req.bucket)
        .bind(&req.open)
        .bind(&req.exp)
        .bind(&req.createtime)
        .bind(&req.uid)
        .bind(&req.hash)
        .bind(&req.deleted)
        .execute(connection.as_mut())
        .await?;
    Ok(())
}

pub async fn get_blob_by_id(id: i64) -> Result<Option<Blob>> {
    let mut connection = db::connection().await?;
    let result = sqlx::query_as::<_, Blob>(r#"
        SELECT 
            id, name, path, size, btype, provider, bucket, open, exp, 
            createtime, uid, hash, deleted
        FROM blobs 
        WHERE id = ? AND deleted = false
        "#)
        .bind(id)
        .fetch_optional(connection.as_mut())
        .await?;
    Ok(result)
}