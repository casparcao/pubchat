CREATE TABLE `friends` (
  `id` BIGINT NOT NULL,
  `name` VARCHAR(255) NOT NULL,
  `avatar` VARCHAR(512) NULL,
  PRIMARY KEY (`id`)
);

CREATE INDEX idx_friends_name ON friends(name);