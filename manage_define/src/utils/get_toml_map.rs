use std::io::Read;

use dependencies_sync::toml;
use dependencies_sync::log::{error};

/// 读取文件为toml_map
pub fn get_toml_map(toml_path: &String) -> Option<toml::map::Map<String, toml::Value>> {
    let mut toml_file = match std::fs::File::open(toml_path) {
        Ok(r) => r,
        Err(_) => {
            error!("初始化数据库文件不存在: {}", toml_path);
            return None;
        }
    };

    let mut toml_string = "".to_string();
    match toml_file.read_to_string(&mut toml_string) {
        Ok(_) => {}
        Err(_) => {
            error!("读取文件错误：{}", toml_path);
            return None;
        }
    }

    let toml_map: toml::map::Map<String, toml::Value> = match toml::from_str(&toml_string) {
        Ok(r) => r,
        Err(_e) => {
            error!("管理定义文件定义错误: {}", toml_path);
            return None;
        }
    };

    Some(toml_map)
}
