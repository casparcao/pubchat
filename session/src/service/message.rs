use anyhow::Result;
use crate::model::message::Message;
use crate::repository::message;


    pub async fn save_message(message: Message) -> Result<()> {
        message::save(&message).await
    }

    pub async fn get_messages_by_session(session: i64, limit: u32) -> Result<Vec<Message>> {
        message::find_by_session(session, limit).await
    }
