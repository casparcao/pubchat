ALTER TABLE `user` 
ADD COLUMN `open_id` VARCHAR(128) NULL AFTER `createtime`,
ADD COLUMN `union_id` VARCHAR(128) NULL AFTER `open_id`,
ADD UNIQUE INDEX `uk_open_id` (`open_id` ASC) ,
ADD UNIQUE INDEX `uk_union_id` (`union_id` ASC) ;
