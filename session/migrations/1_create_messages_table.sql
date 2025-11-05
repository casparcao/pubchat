CREATE TABLE `messages` (
  `id` BIGINT NOT NULL,
  `sender` BIGINT NOT NULL COMMENT '发送者ID',
  `session` BIGINT NOT NULL COMMENT '聊天室ID',
  `mtype` TINYINT NOT NULL DEFAULT '0' COMMENT '消息类型: 0-文本, 1-表情, 2-图片, 3-代码, 4-富文本',
  `content` TEXT NOT NULL COMMENT '消息内容',
  `timestamp` BIGINT NOT NULL COMMENT '消息时间戳',
  `uname` VARCHAR(45) NOT NULL DEFAULT '' COMMENT '发送者昵称',
  `createtime` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  KEY `idx_sender` (`sender`),
  KEY `idx_session` (`session`),
  KEY `idx_timestamp` (`timestamp`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='聊天消息表';