use std::sync::OnceLock;

use dependencies_sync::rust_i18n::{self, t};
use serde_derive::{Deserialize, Serialize};

use configs::{ConfigTrait, get_config};

pub const DATABASE_CONIFIGS_NAME: &str = "database";

static DATABASE_CONFIGS: OnceLock<DatabaseConfigs> = OnceLock::new();

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct DatabaseConfigs {
    pub name: String,
    pub address: String,
    pub port: u16,
}

impl ConfigTrait for DatabaseConfigs {
    fn name() -> &'static str {
        DATABASE_CONIFIGS_NAME
    }
    fn get() -> &'static Self {
        if let Some(configs) = DATABASE_CONFIGS.get() {
            return configs;
        } else {
            let configs = get_config::<DatabaseConfigs>().expect(t!("取得配置失败").as_str());
            DATABASE_CONFIGS.set(configs).expect("设置配置失败");
        }

        DATABASE_CONFIGS.get().unwrap()
    }
}

impl Default for DatabaseConfigs {
    fn default() -> Self {
        DatabaseConfigs {
            name: "cashmere_db".to_string(),
            address: "127.0.0.1".to_string(),
            port: 27017,
        }
    }
}