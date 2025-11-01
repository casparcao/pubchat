CREATE TABLE `contact` (
  `id` BIGINT NOT NULL,
  `here` BIGINT NOT NULL,
  `there` BIGINT NOT NULL,
  `name` VARCHAR(255) NOT NULL,
  `avatar` VARCHAR(512) NULL,
  PRIMARY KEY (`id`)
);

CREATE INDEX idx_here ON contact(here);