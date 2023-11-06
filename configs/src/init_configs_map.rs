use std::io::Read;
use std::path::PathBuf;

use dependencies_sync::log;
use dependencies_sync::rust_i18n::{self, t};

use crate::{get_configs_map, get_configs_file_path};



pub fn init_configs_map() -> Result<(), String> {
    let configs_path: &String = &get_configs_file_path().to_string(); 
    let mut config_file =
        std::fs::File::open(PathBuf::from(configs_path)).unwrap_or_else(|_| { panic!("{}", t!("配置文件不存在").to_string()) });

    let mut config_str = "".to_string();
    config_file
        .read_to_string(&mut config_str)
        .expect(t!("读取配置文件失败").as_str());

    let configs_table: toml::Table = toml::from_str(config_str.as_str()).unwrap();
    {
        let configs_map = get_configs_map();
        let mut configs_map = configs_map.write();
        configs_table.into_iter().for_each(|(k, v)| {
            configs_map.insert(k, v.try_into().unwrap());
        });
    }

    log::info!("{}: {:?}", t!("成功加载配置"), configs_path);
    
    Ok(())
}
