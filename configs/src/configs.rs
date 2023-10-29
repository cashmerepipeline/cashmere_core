use dependencies_sync::rust_i18n::{self, t};
use serde_derive::Deserialize;
use std::path::PathBuf;
use std::sync::Arc;

use crate::data_server_configs::DataServerConfigs;
use crate::database_configs::DatabaseConfigs;
use crate::load_configs;
use crate::{ServerConfigs, TlsConfigs};

#[derive(Deserialize)]
pub struct Configs {
    pub server: ServerConfigs,
    pub database: DatabaseConfigs,
    pub tls: TlsConfigs,
    pub data_server: DataServerConfigs,
}

static mut CONFIGS_FILE_PATH: Option<Arc<String>> = None;
static mut CONFIGS: Option<Arc<Configs>> = None;

pub fn init_configs_path(configs_path: String) -> Result<(), ()> {
    unsafe {
        if CONFIGS_FILE_PATH.is_none() {
            CONFIGS_FILE_PATH.replace(Arc::new(configs_path));
            Ok(())
        } else {
            Err(())
        }
    }
}

pub fn get_configs_path() -> &'static String {
    unsafe {
        if CONFIGS_FILE_PATH.is_some() {
            CONFIGS_FILE_PATH.as_ref().unwrap()
        } else {
            panic!("配置文件不存在: {}", CONFIGS_FILE_PATH.as_ref().unwrap())
        }
    }
}

pub fn get_configs() -> &'static Configs {
    unsafe {
        if CONFIGS.is_some() {
            CONFIGS.as_ref().unwrap()
        } else {
            let path_str = get_configs_path();
            let configs_path = PathBuf::from(path_str);
            if configs_path.exists() {
                CONFIGS.get_or_insert(load_configs(configs_path).unwrap());
                CONFIGS.as_ref().unwrap()
            } else {
                panic!("{}: {}", t!("配置文件不存在"), path_str);
            }
        }
    }
}

