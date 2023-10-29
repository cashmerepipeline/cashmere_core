use crate::get_configs_map;

use super::Configs;
use dependencies_sync::log;
use dependencies_sync::rust_i18n::{self, t};
use std::io::Read;
use std::path::PathBuf;
use std::sync::Arc;
use toml;

pub fn load_configs(configs_path: PathBuf) -> Option<Arc<Configs>> {
    let mut config_file =
        std::fs::File::open(&configs_path).expect(format!("{}", t!("配置文件不存在")).as_ref());

    let mut config_str = "".to_string();
    config_file
        .read_to_string(&mut config_str)
        .expect("配置文件错误");

    let configs: Configs =
        toml::from_str(config_str.as_str()).expect(format!("{}", t!("构建toml失败")).as_ref());

    let configs_table: toml::Table = toml::from_str(config_str.as_str()).unwrap();
    {
        let configs_map = get_configs_map();
        let mut configs_map = configs_map.write();
        configs_table.into_iter().for_each(|(k, v)| {
            configs_map.insert(k, v.try_into().unwrap());
        });
    }

    log::info!("{}: {:?}", t!("成功加载配置"), configs_path);

    Some(Arc::new(configs))
}
