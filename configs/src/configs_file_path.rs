use dependencies_sync::{rust_i18n::{self, t}, log};


use std::sync::Arc;

static mut CONFIGS_FILE_PATH: Option<Arc<String>> = None;

pub fn init_configs_file_path(configs_path: String) -> Result<(), String> {
    unsafe {
        if CONFIGS_FILE_PATH.is_none() {
            log::info!("{}: {}", t!("初始化设置文件路径"), configs_path);

            CONFIGS_FILE_PATH.replace(Arc::new(configs_path));
            return    Ok(())
        }
        
        Err(t!("配置文件路径已初始化"))
    }
}

pub fn get_configs_file_path() -> &'static String {
    unsafe {
        if CONFIGS_FILE_PATH.is_some() {
            CONFIGS_FILE_PATH.as_ref().unwrap()
        } else {
            panic!(
                "{}: {}",
                t!("配置文件不存在"),
                CONFIGS_FILE_PATH.as_ref().unwrap()
            )
        }
    }
}
