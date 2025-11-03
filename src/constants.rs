use std::sync::OnceLock;

pub static DATA_DIR: OnceLock<String> = OnceLock::new();
pub static CACHE_DIR: OnceLock<String> = OnceLock::new();
