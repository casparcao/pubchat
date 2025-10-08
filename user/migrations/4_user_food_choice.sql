CREATE TABLE `user_food_choice` (
  `id` BIGINT NOT NULL,
  `uid` BIGINT NOT NULL,
  `fid` BIGINT NOT NULL,
  `createtime` DATETIME NOT NULL DEFAULT current_timestamp,
  PRIMARY KEY (`id`))
COMMENT = '用户决定吃的食物';

ALTER TABLE `food`
ADD COLUMN `pick_count` INT NOT NULL DEFAULT 0 COMMENT '被选择的次数。' AFTER `name`;
