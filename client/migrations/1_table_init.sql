CREATE TABLE `messages` (
  `id` BIGINT NOT NULL,
  `speaker_id` BIGINT NOT NULL ,
  `receiver_id` BIGINT NOT NULL ,
  `room_id` BIGINT NOT NULL ,
  `message_type` TINYINT NOT NULL DEFAULT '0',
  `content` TEXT NOT NULL ,
  `timestamp` BIGINT NOT NULL ,
  `nickname` VARCHAR(45) NOT NULL DEFAULT '' ,
  PRIMARY KEY (`id`)
) ;

CREATE INDEX idx_speaker ON messages(speaker_id);
CREATE INDEX idx_room ON messages(room_id);
CREATE INDEX idx_timestamp ON messages(`timestamp`);
