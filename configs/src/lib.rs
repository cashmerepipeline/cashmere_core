/*
Author: 闫刚 (yes7rose@sina.com)
lib.rs (c) 2020
Desc: 设置取得
Created:  2020-09-24T06:56:14.907Z
Modified: !date!
*/

/*
使用单例模式创建数据库 configs
所有操作只使用一个 configs, 需要进一步测试
*/

use serde_derive::Deserialize;
use std::sync::Arc;
use std::io::Read;

#[derive(Deserialize, Clone)]
pub struct ServerConfigs {
    pub root_dir: String,
    pub address: String,
    pub port: String,
    pub secret_code: String,
    pub use_tls: bool,
    pub login_limit: u8,
    // pub managers_path: Vec<String>,
    pub events_dbs_dir: String,
    pub language_code: String,
    pub log_dir: String,
}

#[derive(Deserialize, Clone)]
pub struct DatabaseConfigs {
    pub name: String,
    pub address: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct TlsConfigs {
    pub server_key_path: String,
    pub server_ca_path: String,
    pub client_ca_path: String,
}

#[derive(Deserialize, Clone)]
pub struct DataConfigs {
    // 数据根目录
    pub root: String,
    // 文件最大大小
    max_file_size: u64,
    // 文件集最大数量
    max_set_file_number: u32,
    // 文件序列最大数量
    max_sequence_file_number: u32,
    // 上传数据块最大数量
    max_chunk_size: u32,
}

#[derive(Deserialize)]
pub struct Configs {
    pub server: ServerConfigs,
    pub database: DatabaseConfigs,
    pub tls: TlsConfigs,
    pub data: DataConfigs,
}

static mut CONFIGS_FILE_PATH: Option<Arc<String>> = None;
static mut CONFIGS: Option<Arc<Configs>> = None;

pub fn init_configs_path(configs_path: String) -> Result<(), ()>{
    unsafe {
        if CONFIGS_FILE_PATH.is_none() {
            CONFIGS_FILE_PATH.replace(Arc::new(configs_path));
            Ok(())
        }else {
        Err(())
        }
    }
}

pub fn get_configs_path() -> &'static String{
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
            CONFIGS.get_or_insert(read_configs().unwrap());
            CONFIGS.as_ref().unwrap()
        }
    }
}

fn read_configs() -> Option<Arc<Configs>> {
    let mut config_file = std::fs::File::open(get_configs_path())
        .expect("配置文件不存在");
    let mut config_str = "".to_string();
    config_file
        .read_to_string(&mut config_str)
        .expect("配置文件错误");

    let _configs: Configs = toml::from_str(config_str.as_str()).expect("构建toml失败");

    Some(Arc::new(_configs))
}

/// 取得服务器设置
pub fn get_server_configs() -> ServerConfigs{
    let configs = get_configs();
    configs.server.clone()
}

/// 取得数据库设置
pub fn get_database_configs() -> DatabaseConfigs{
    let configs = get_configs();
    configs.database.clone()
}

/// 取得服务器语言设置
pub fn get_language_code() -> String {
    let configs = get_configs();
    configs.server.language_code.clone()
}

/// 取得数据设置
pub fn get_data_configs() -> DataConfigs{
    let configs = get_configs();
    configs.data.clone()
}

#[cfg(test)]
mod tests {
    use crate::get_configs;

    #[test]
    fn get_configs_test() {
        let configs = get_configs();
        assert_eq!(configs.database.address, "localhost");
        assert_eq!(configs.database.port, 27017);
        assert_eq!(configs.server.address, "127.0.0.1");
        assert_eq!(configs.server.port, "8800");
        assert_eq!(configs.server.use_tls, false);
        assert_eq!(configs.server.login_limit, 2);
    }
}
