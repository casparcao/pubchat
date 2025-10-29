//缓存ui聊天框的聊天列表

use std::collections::HashMap;

use crate::repository::message::Message;

pub(crate) struct Cache {
    pub(crate) messages: HashMap<i64, Vec<Message>>,
}

impl Cache {
    pub(crate) fn new() -> Self {
        Self {
            messages: HashMap::new(),
        }
    }

    pub(crate) fn add(&mut self, room_id: i64, message: Message) {
        if let Some(messages) = self.messages.get_mut(&room_id) {
            messages.push(message);
        } else {
            self.messages.insert(room_id, vec![message]);
        }
    }

    pub(crate) fn get(&self, room_id: i64) -> Option<&Vec<Message>> {
        self.messages.get(&room_id)
    }

}