/*
Author: 闫刚 (yes7rose@sina.com)
lib.rs (c) 2020
Desc: 设置取得
Created:  2020-09-24T06:56:14.907Z
Modified: !date!
*/

/*
使用单例模式创建数据库 configs
所有操作只使用一个 configs
*/

use dependencies_sync::once_cell;
use dependencies_sync::rust_i18n::{self, i18n};
i18n!("locales");

mod get_language_code;
mod database_configs;
mod server_configs;
mod data_server_configs;
mod tls_configs;
mod configs;
mod read_configs_file_path;

mod get_server_configs;
mod get_database_configs;
mod get_data_server_configs;
mod config_trait;
mod configs_map;

pub use config_trait::*;
pub use configs_map::*;

pub use configs::*;
pub use get_language_code::*;
pub use database_configs::*;
pub use server_configs::*;
pub use data_server_configs::*;
pub use tls_configs::*;

pub use get_server_configs::*;
pub use get_database_configs::*;
pub use get_data_server_configs::*;

pub use read_configs_file_path::*;

#[cfg(test)]
mod tests {
    use crate::configs::get_configs;

    #[test]
    fn get_configs_test() {
        let configs = get_configs();
        assert_eq!(configs.database.address, "localhost");
        assert_eq!(configs.database.port, 27017);
        assert_eq!(configs.server.address, "127.0.0.1");
        assert_eq!(configs.server.port, "8800");
        assert!(!configs.server.use_tls);
        assert_eq!(configs.server.login_limit, 2);
    }
}
