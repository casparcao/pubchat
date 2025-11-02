CREATE TABLE `session` (
  `id` BIGINT NOT NULL,
  `uid` BIGINT NOT NULL,
  `sid` BIGINT NOT NULL,
  `name` VARCHAR(255) NOT NULL,
  `avatar` VARCHAR(512) NULL,
  PRIMARY KEY (`id`)
);

CREATE INDEX idx_user_id ON session(uid);