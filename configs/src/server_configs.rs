use std::sync::OnceLock;

use dependencies_sync::rust_i18n::{self, t};
use serde_derive::{Deserialize, Serialize};

use crate::{ConfigTrait, get_config};

pub const SERVER_CONFIGS_NAME: &str = "server";
static SERVER_CONFIGS: OnceLock<ServerConfigs> = OnceLock::new();

#[derive(Deserialize, Clone, Serialize, Debug)]
pub struct ServerConfigs {
    pub root_dir: String,
    pub address: String,
    pub port: String,
    pub secret_code: String,
    pub use_tls: bool,
    pub login_limit: u8,
    pub language_code: String,
    pub log_dir: String,
    pub log_level: String,
    pub max_page_size: u16,
    pub root_id: String,
    pub admin_group: String,
}

impl ConfigTrait for ServerConfigs {
    fn name() -> &'static str {
        SERVER_CONFIGS_NAME
    }
    fn get() -> &'static Self {
        if let Some(configs) = SERVER_CONFIGS.get() {
            return configs;
        } else{
            let configs = get_config::<ServerConfigs>().expect(t!("取得配置失败").as_str());
            SERVER_CONFIGS.set(configs).expect("设置配置失败");
        }
        
        SERVER_CONFIGS.get().unwrap()
    }
}

impl Default for ServerConfigs {
    fn default() -> Self {
        ServerConfigs {
            root_dir: ".".to_string(),
            address: "127.0.0.1".to_string(),
            port: "8800".to_string(),
            secret_code: "$camshere*soft*warm*smooth*beauty$".to_string(),
            use_tls: false,
            login_limit: 2,
            language_code: "zh".to_string(),
            log_dir: "log".to_string(),
            log_level: "info".to_string(),
            max_page_size: 20,
            root_id:"8610000000000".to_string(),
            admin_group: "admin".to_string(),
        }
    }
}
