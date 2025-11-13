use std::fmt::{Display, Formatter};

impl From<crate::core::message::chrt::Message> for crate::core::message::chrs::Message { 
    fn from(value: crate::core::message::chrt::Message) -> Self { 
        match value {
            crate::core::message::chrt::Message::Text(text) => Self::Text(text),
            crate::core::message::chrt::Message::Blob(file) => Self::Blob(file),
        }
    }
}

impl From<&crate::core::message::chrt::Message> for crate::core::message::chrs::Message { 
    fn from(value: &crate::core::message::chrt::Message) -> Self { 
        match value {
            crate::core::message::chrt::Message::Text(text) => Self::Text(text.clone()),
            crate::core::message::chrt::Message::Blob(file) => Self::Blob(file.clone()),
        }
    }
}

impl Display for crate::core::message::chrt::Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        match self {
            crate::core::message::chrt::Message::Text(text) => write!(f, "{}", text.text),
            crate::core::message::chrt::Message::Blob(file) => write!(f, "[File] {} (size: {}, exp: {}, download id: {})", 
                file.name, file.size, file.exp.as_ref().unwrap_or(&"never".to_string()), base62::encode(file.id as u64)),
        }
    }
}

impl Display for crate::core::message::chrs::Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        match self {
            crate::core::message::chrs::Message::Text(text) => write!(f, "{}", text.text),
            crate::core::message::chrs::Message::Blob(file) => write!(f, "[File] {} (size: {}, exp: {}, download id: {})", 
                file.name, file.size, file.exp.as_ref().unwrap_or(&"never".to_string()), base62::encode(file.id as u64)),
        }
    }
}