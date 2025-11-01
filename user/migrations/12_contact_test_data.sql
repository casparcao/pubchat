INSERT INTO `user` (`id`,`name`,`password`,`gender`,`age`,`createtime`) VALUES (1,'alice','alice','F',25,'2024-04-02 22:57:49');
INSERT INTO `user` (`id`,`name`,`password`,`gender`,`age`,`createtime`) VALUES (2,'bob','bob','M',30,'2024-04-02 22:57:49');
-- 添加测试好友关系数据
INSERT INTO `contact` (`id`, `here`, `there`, `status`, `createtime`, `updatetime`) 
VALUES 
(1, 0, 1, 1, NOW(), NOW()),  -- admin的好友alice
(2, 0, 2, 1, NOW(), NOW()),  -- admin的好友bob
(3, 1, 0, 1, NOW(), NOW()),  -- alice的好友admin
(4, 2, 0, 1, NOW(), NOW());  -- bob的好友admin