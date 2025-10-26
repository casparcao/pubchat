use anyhow::Result;
use crate::model::message::Message;
use crate::repository::message;


    pub async fn save_message(message: Message) -> Result<()> {
        message::save(&message).await
    }

    pub async fn get_messages_by_room(room_id: i64, limit: u32) -> Result<Vec<Message>> {
        message::find_by_room_id(room_id, limit).await
    }

    pub async fn get_messages_by_speaker(speaker_id: i64, limit: u32) -> Result<Vec<Message>> {
        message::find_by_speaker_id(speaker_id, limit).await
    }