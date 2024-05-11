use std::sync::OnceLock;

use dependencies_sync::rust_i18n::{self, t};
use serde_derive::{Deserialize, Serialize};

use crate::{ConfigTrait, get_config};

pub const MANAGES_CONFIGS_NAME: &str = "manages";
static SERVER_CONFIGS: OnceLock<ManagesConfigs> = OnceLock::new();

#[derive(Deserialize, Clone, Serialize, Debug)]
pub struct ManagesConfigs{
  pub public_manages: Vec<&str>,
  pub forbid_manages: Vec<&str>,
}

impl ConfigTrait for ManagesConfigs {
    fn name() -> &'static str {
        MANAGES_CONFIGS_NAME
    }
    fn get() -> &'static Self {
        if let Some(configs) = SERVER_CONFIGS.get() {
            return configs;
        } else{
            let configs = get_config::<ManagesConfigs>().expect(t!("取得配置失败").as_str());
            SERVER_CONFIGS.set(configs).expect("设置配置失败");
        }
        
        SERVER_CONFIGS.get().unwrap()
    }
}

impl Default for ManagesConfigs {
    fn default() -> Self {
        ManagesConfigs {
          // TODO: 
        }
    }
}
