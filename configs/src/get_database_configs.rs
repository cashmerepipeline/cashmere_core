use crate::DatabaseConfigs;

/// 取得数据库设置
pub fn get_database_configs() -> DatabaseConfigs {
    let configs = crate::get_configs();
    configs.database.clone()
}
