use std::sync::OnceLock;

pub mod message;
pub mod friends;
pub mod session;

pub use message::Cache as MessageCache;
pub use session::Cache as SessionCache;
pub use friends::Cache as FriendCache;

pub static MESSAGE_CACHE: OnceLock<MessageCache> = OnceLock::new();
pub static SESSION_CACHE: OnceLock<SessionCache> = OnceLock::new();
pub static FRIEND_CACHE: OnceLock<FriendCache> = OnceLock::new();

pub fn friends_cache() -> &'static FriendCache {
    FRIEND_CACHE.get().unwrap()
}

pub fn init() {
    MESSAGE_CACHE.get_or_init(|| MessageCache::new());
    FRIEND_CACHE.get_or_init(|| FriendCache::new());
    SESSION_CACHE.get_or_init(|| SessionCache::new());
}

pub fn message_cache() -> &'static MessageCache {
    MESSAGE_CACHE.get().unwrap()
}

pub fn session_cache() -> &'static SessionCache {
    SESSION_CACHE.get().unwrap()
}