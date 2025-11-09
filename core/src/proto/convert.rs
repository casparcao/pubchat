use std::fmt::{Display, Formatter};

impl From<crate::proto::message::chrt::Message> for crate::proto::message::chrs::Message { 
    fn from(value: crate::proto::message::chrt::Message) -> Self { 
        match value {
            crate::proto::message::chrt::Message::Text(text) => Self::Text(text),
            crate::proto::message::chrt::Message::Blob(file) => Self::Blob(file),
        }
    }
}

impl From<&crate::proto::message::chrt::Message> for crate::proto::message::chrs::Message { 
    fn from(value: &crate::proto::message::chrt::Message) -> Self { 
        match value {
            crate::proto::message::chrt::Message::Text(text) => Self::Text(text.clone()),
            crate::proto::message::chrt::Message::Blob(file) => Self::Blob(file.clone()),
        }
    }
}

impl Display for crate::proto::message::chrt::Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        match self {
            crate::proto::message::chrt::Message::Text(text) => write!(f, "{}", text.text),
            crate::proto::message::chrt::Message::Blob(file) => write!(f, "[File] {} (size: {}, exp: {}, download id: {})", 
                file.name, file.size, file.exp.as_ref().unwrap_or(&"never".to_string()), base62::encode(file.id as u64)),
        }
    }
}

impl Display for crate::proto::message::chrs::Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        match self {
            crate::proto::message::chrs::Message::Text(text) => write!(f, "{}", text.text),
            crate::proto::message::chrs::Message::Blob(file) => write!(f, "[File] {} (size: {}, exp: {}, download id: {})", 
                file.name, file.size, file.exp.as_ref().unwrap_or(&"never".to_string()), base62::encode(file.id as u64)),
        }
    }
}