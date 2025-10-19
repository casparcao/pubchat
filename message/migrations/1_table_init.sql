CREATE TABLE `module` (
  `id` bigint NOT NULL,
  `name` varchar(45) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `code` varchar(45) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `createtime` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `desc` varchar(45) COLLATE utf8mb4_unicode_ci DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='菜单模块，功能模块，用户角色权限控制，由开发人员维护，主要对应菜单页面，功能按钮';

CREATE TABLE `org` (
  `id` bigint NOT NULL,
  `name` varchar(45) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '组织名',
  `code` varchar(45) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '' COMMENT '组织代码',
  `createtime` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='组织/企业';

CREATE TABLE `org_user_relation` (
  `id` bigint NOT NULL,
  `uid` bigint NOT NULL DEFAULT '0',
  `oid` bigint NOT NULL DEFAULT '0',
  `createtime` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `role` (
  `id` bigint NOT NULL,
  `name` varchar(45) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `code` varchar(20) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `builtin` tinyint NOT NULL DEFAULT '0' COMMENT '是否是内置角色',
  `createtime` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `oid` bigint NOT NULL DEFAULT '0' COMMENT '所属组织（允许组织自建角色）',
  `creator` bigint NOT NULL DEFAULT '0',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `role_module_relation` (
  `id` bigint NOT NULL,
  `rid` bigint NOT NULL DEFAULT '0',
  `mid` bigint NOT NULL DEFAULT '0',
  `createtime` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_rid_mid` (`rid`,`mid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `user` (
  `id` bigint NOT NULL,
  `name` varchar(45) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `password` varchar(45) COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT '',
  `gender` enum('M','F','U') COLLATE utf8mb4_unicode_ci NOT NULL DEFAULT 'U',
  `age` tinyint NOT NULL DEFAULT '0',
  `createtime` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

CREATE TABLE `user_role_relation` (
  `id` bigint NOT NULL,
  `uid` bigint NOT NULL DEFAULT '0',
  `rid` bigint NOT NULL DEFAULT '0',
  `createtime` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_uid_rid` (`uid`,`rid`),
  KEY `uk_rid` (`rid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

INSERT INTO `role` (`id`,`name`,`code`,`builtin`,`createtime`,`oid`,`creator`) VALUES (0,'超级管理员','SM',1,'2024-04-02 22:57:08',0,0);
INSERT INTO `role` (`id`,`name`,`code`,`builtin`,`createtime`,`oid`,`creator`) VALUES (1,'组织管理员','OM',1,'2024-04-02 22:57:08',0,0);
INSERT INTO `role` (`id`,`name`,`code`,`builtin`,`createtime`,`oid`,`creator`) VALUES (2,'普通用户','NU',1,'2024-04-02 22:57:08',0,0);

INSERT INTO `user_role_relation` (`id`,`uid`,`rid`,`createtime`) VALUES (0,0,0,'2024-04-02 22:58:04');

INSERT INTO `user` (`id`,`name`,`password`,`gender`,`age`,`createtime`) VALUES (0,'admin','admin','M',0,'2024-04-02 22:57:49');