use crate::{ServerConfigs, SERVER_CONFIGS_NAME};

/// 取得服务器设置
pub fn get_server_configs() -> ServerConfigs {
    let configs = crate::get_config::<ServerConfigs>().unwrap();
    configs
}
