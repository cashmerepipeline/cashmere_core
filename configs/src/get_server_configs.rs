use crate::ServerConfigs;

/// 取得服务器设置
pub fn get_server_configs() -> &'static ServerConfigs {
    let configs = crate::get_configs();
    &configs.server
}
