use core::extension::ExtensionManager;
use std::sync::OnceLock;

pub static EM: OnceLock<ExtensionManager> = OnceLock::new();

pub(crate) fn init() {
    EM.get_or_init(|| {ExtensionManager::new()});
}

pub(crate) fn get() -> &'static ExtensionManager {
    EM.get().unwrap()
}