use configs::ConfigTrait;

use crate::DatabaseConfigs;

/// zh: 从配置文件中获取数据库名称
/// en: Get the database name from the configuration file
pub fn get_database_name() -> String {
    let database_configs = DatabaseConfigs::get();

    database_configs.name.clone()
}
