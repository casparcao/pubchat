CREATE TABLE `session` (
  `id` BIGINT NOT NULL,
  `user_id` BIGINT NOT NULL,
  `session_id` BIGINT NOT NULL,
  `name` VARCHAR(255) NOT NULL,
  `avatar` VARCHAR(512) NULL,
  PRIMARY KEY (`id`)
);

CREATE INDEX idx_user_id ON contact(user_id);