use std::sync::OnceLock;

pub mod message;
pub mod friends;

pub use message::Cache as MessageCache;
pub use friends::Cache as FriendCache;

pub static MESSAGE_CACHE: OnceLock<MessageCache> = OnceLock::new();

pub static FRIEND_CACHE: OnceLock<FriendCache> = OnceLock::new();

pub fn friends_cache() -> &'static FriendCache {
    FRIEND_CACHE.get().unwrap()
}

pub fn init() {
    MESSAGE_CACHE.get_or_init(|| MessageCache::new());
    FRIEND_CACHE.get_or_init(|| FriendCache::new());
}

pub fn message_cache() -> &'static MessageCache {
    MESSAGE_CACHE.get().unwrap()
}