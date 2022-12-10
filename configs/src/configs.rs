use std::sync::Arc;
use serde_derive::Deserialize;
use std::io::Read;
use crate::database_configs::DatabaseConfigs;
use crate::{ServerConfigs, TlsConfigs};
use crate::data_server_configs::DataServerConfigs;

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
            panic!("配置文件不存在")
        }
    }
}

pub fn get_configs() -> &'static Configs {
    unsafe {
        if CONFIGS.is_some() {
            CONFIGS.as_ref().unwrap()
        } else {
            CONFIGS.get_or_insert(load_configs().unwrap());
            CONFIGS.as_ref().unwrap()
        }
    }
}

fn load_configs() -> Option<Arc<Configs>> {
    let mut config_file = std::fs::File::open(get_configs_path()).expect("配置文件不存在");
    let mut config_str = "".to_string();
    config_file
        .read_to_string(&mut config_str)
        .expect("配置文件错误");

    let _configs: Configs = toml::from_str(config_str.as_str()).expect("构建toml失败");

    Some(Arc::new(_configs))
}
