-- 会话表
CREATE TABLE `sessions` (
  `id` BIGINT NOT NULL,
  `name` VARCHAR(100) NOT NULL COMMENT '会话名称',
  `creator` BIGINT NOT NULL COMMENT '创建人ID',
  `createtime` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updatetime` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  KEY `idx_creator` (`creator`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='会话表';

-- 用户会话关联表
CREATE TABLE `user_sessions` (
  `id` BIGINT NOT NULL,
  `uid` BIGINT NOT NULL COMMENT '用户ID',
  `uname` VARCHAR(100) NOT NULL COMMENT '用户名称',
  `sid` BIGINT NOT NULL COMMENT '会话ID',
  `role` TINYINT NOT NULL DEFAULT '0' COMMENT '角色: 0-普通成员, 1-管理员',
  `jointime` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_user_session` (`uid`, `sid`),
  KEY `idx_session` (`sid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='用户会话关联表';