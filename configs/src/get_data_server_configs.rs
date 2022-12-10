use crate::DataServerConfigs;

/// 取得数据设置
pub fn get_data_server_configs() -> &'static DataServerConfigs {
    let configs = crate::get_configs();
    &configs.data_server
}
