CREATE TABLE blobs (
    -- 主键：全局唯一 ID
    id BIGINT NOT NULL ,
    -- 客户端原始文件名（用于下载时建议的文件名）
    name varchar(1024) NOT NULL,
    -- 存储在后端的真实路径或对象 key（如 s3://bucket/path/to/file）
    path TEXT NOT NULL ,
    -- 文件大小（字节）
    size BIGINT NOT NULL DEFAULT 0,
    -- MIME 类型（如 image/jpeg, application/pdf）
    `btype` varchar(64) NOT NULL DEFAULT 'application/octet-stream',
    -- 存储提供商（枚举：local, s3, minio, aliyun_oss, tencent_cos, azure_blob 等）
    provider VARCHAR(32) NOT NULL DEFAULT 'local',
    -- 可选：所属 bucket 名称（适用于 S3/OSS 等）
    bucket TEXT,
    -- 是否公开可读？
    -- true  可直接通过 URL 访问（如 CDN）
    -- false: 必须带 token 或登录才能下载
    `open` tinyint NOT NULL DEFAULT 0,
    -- 过期时间（NULL 表示永不过期）
    exp datetime,
	-- 上传时间
    createtime datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- 上传者用户 ID（可选，如果你有用户系统）
    uid bigint not null default 0,
    -- 内容哈希（推荐 SHA-256 或 BLAKE3），用于去重和完整性校验
    hash CHAR(64),
    -- 软删除标记（逻辑删除）
    deleted tinyint NOT NULL DEFAULT 0
);

-- 按上传者查询
CREATE INDEX idx_blobs_uploader_id ON blobs(uid);
-- 按过期时间清理用
CREATE INDEX idx_blobs_expires_at ON blobs(exp) ;
-- 按创建时间排序（常用）
CREATE INDEX idx_blobs_created_at ON blobs(createtime DESC) ;
-- 去重查询加速
CREATE INDEX idx_blobs_content_hash ON blobs(hash) ;