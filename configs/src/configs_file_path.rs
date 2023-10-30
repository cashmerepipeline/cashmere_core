use dependencies_sync::rust_i18n::{self, t};
use serde_derive::Deserialize;
use std::path::PathBuf;
use std::sync::Arc;

static mut CONFIGS_FILE_PATH: Option<Arc<String>> = None;

pub fn init_configs_file_path(configs_path: String) -> Result<(), ()> {
    unsafe {
        if CONFIGS_FILE_PATH.is_none() {
            CONFIGS_FILE_PATH.replace(Arc::new(configs_path));
            Ok(())
        } else {
            Err(())
        }
    }
}

pub fn get_configs_file_path() -> &'static String {
    unsafe {
        if CONFIGS_FILE_PATH.is_some() {
            CONFIGS_FILE_PATH.as_ref().unwrap()
        } else {
            panic!("配置文件不存在: {}", CONFIGS_FILE_PATH.as_ref().unwrap())
        }
    }
}
