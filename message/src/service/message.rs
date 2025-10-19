use anyhow::Result;
use crate::model::message::Message;
use crate::repository::message::MessageRepository;

pub struct MessageService {
    repository: MessageRepository,
}

impl MessageService {
    pub fn new(repository: MessageRepository) -> Self {
        Self { repository }
    }

    pub async fn save_message(&self, message: Message) -> Result<()> {
        self.repository.save(&message).await
    }

    pub async fn get_messages_by_room(&self, room_id: i64, limit: u32) -> Result<Vec<Message>> {
        self.repository.find_by_room_id(room_id, limit).await
    }

    pub async fn get_messages_by_speaker(&self, speaker_id: i64, limit: u32) -> Result<Vec<Message>> {
        self.repository.find_by_speaker_id(speaker_id, limit).await
    }
}