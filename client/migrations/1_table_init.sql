CREATE TABLE `messages` (
  `id` BIGINT NOT NULL,
  `sender` BIGINT NOT NULL ,
  `receiver` BIGINT NOT NULL ,
  `session` BIGINT NOT NULL ,
  `mtype` TINYINT NOT NULL DEFAULT '0',
  `content` TEXT NOT NULL ,
  `timestamp` BIGINT NOT NULL ,
  `uname` VARCHAR(45) NOT NULL DEFAULT '' ,
  PRIMARY KEY (`id`)
) ;

CREATE INDEX idx_sender ON messages(sender);
CREATE INDEX idx_session ON messages(session);
CREATE INDEX idx_timestamp ON messages(`timestamp`);
