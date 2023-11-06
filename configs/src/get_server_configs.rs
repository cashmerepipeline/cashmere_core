use crate::{ServerConfigs};

/// 取得服务器设置
pub fn get_server_configs() -> ServerConfigs {
    
    crate::get_config::<ServerConfigs>().unwrap()
}
