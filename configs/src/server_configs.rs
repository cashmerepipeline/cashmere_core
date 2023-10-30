use serde_derive::{Deserialize, Serialize};

use crate::ConfigTrait;

pub const SERVER_CONFIGS_NAME: &str = "server";

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
}

impl ConfigTrait for ServerConfigs {
    fn name() -> &'static str {
        return SERVER_CONFIGS_NAME;
    }
}

impl Default for ServerConfigs {
    fn default() -> Self {
        return ServerConfigs {
            root_dir: ".".to_string(),
            address: "127.0.0.1".to_string(),
            port: "8800".to_string(),
            secret_code: "$camshere*soft*warm*smooth*beauty$".to_string(),
            use_tls: false,
            login_limit: 2,
            language_code: "zh".to_string(),
            log_dir: "log".to_string(),
            log_level: "info".to_string(),
        };
    }
}
