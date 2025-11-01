CREATE TABLE `contact` (
  `id` bigint NOT NULL,
  `here` bigint NOT NULL COMMENT '用户ID',
  `there` bigint NOT NULL COMMENT '好友ID',
  `status` tinyint NOT NULL DEFAULT '1' COMMENT '关系状态: 1-正常, 2-拉黑',
  `createtime` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `updatetime` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_here_there` (`here`,`there`),
  KEY `idx_here` (`here`),
  KEY `idx_there` (`there`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='好友关系表';