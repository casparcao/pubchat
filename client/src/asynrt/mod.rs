use std::sync::OnceLock;

use tokio::runtime::Runtime;


pub static RT: OnceLock<Runtime> = OnceLock::new();

pub(crate) fn init() {
    RT.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
    });
}

pub(crate) fn get() -> &'static Runtime {
    RT.get().unwrap()
}