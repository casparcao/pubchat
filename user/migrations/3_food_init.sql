CREATE TABLE `food` (
  `id` BIGINT NOT NULL,
  `name` VARCHAR(45) NOT NULL DEFAULT '',
  PRIMARY KEY (`id`));

CREATE TABLE `tag` (
  `id` BIGINT NOT NULL,
  `name` VARCHAR(45) NOT NULL,
  PRIMARY KEY (`id`));

CREATE TABLE `food_tag` (
  `id` BIGINT NOT NULL,
  `fid` BIGINT NOT NULL,
  `tid` BIGINT NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE INDEX `uk_f_t` (`tid` ASC, `fid` ASC) ,
  INDEX `idx_f` (`fid` ASC));
