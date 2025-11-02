use std::sync::OnceLock;

pub mod message;
pub mod contact;
pub mod session;

pub use message::Cache as MessageCache;
pub use session::Cache as SessionCache;
pub use contact::Cache as ContactCache;

pub static MESSAGE_CACHE: OnceLock<MessageCache> = OnceLock::new();
pub static SESSION_CACHE: OnceLock<SessionCache> = OnceLock::new();
pub static CONTACT_CACHE: OnceLock<ContactCache> = OnceLock::new();

pub fn contact_cache() -> &'static ContactCache {
    CONTACT_CACHE.get().unwrap()
}

pub fn init() {
    MESSAGE_CACHE.get_or_init(|| MessageCache::new());
    CONTACT_CACHE.get_or_init(|| ContactCache::new());
    SESSION_CACHE.get_or_init(|| SessionCache::new());
}

pub fn message_cache() -> &'static MessageCache {
    MESSAGE_CACHE.get().unwrap()
}

pub fn session_cache() -> &'static SessionCache {
    SESSION_CACHE.get().unwrap()
}