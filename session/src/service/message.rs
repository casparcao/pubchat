use anyhow::Result;
use crate::model::message::Message;
use crate::repository::message;
use core::api::types::message::Message as MessageResponse;


pub async fn save_message(message: Message) -> Result<()> {
    message::save(&message).await
}

pub async fn get_messages_by_session(session: i64, limit: u32) -> Result<Vec<MessageResponse>> {
    let m = message::find_by_session(session, limit).await?;
    Ok(m.into_iter().map(|m| MessageResponse {
        id: m.id,
        sender: m.sender,
        receiver: 0,
        session: m.session,
        mtype: m.mtype,
        uname: m.uname,
        content: m.content,
        timestamp: m.timestamp,
    }).collect())
}
