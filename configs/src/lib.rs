/*
Author: 闫刚 (yes7rose@sina.com)
lib.rs (c) 2020
Desc: 设置取得
Created:  2020-09-24T06:56:14.907Z
Modified: !date!
*/

/*
使用单例模式创建数据库 configs
所有操作只使用一个 configs, 需要进一步测试
*/

mod get_language_code;
mod database_configs;
mod server_configs;
mod data_server_configs;
mod tls_configs;
mod configs;

mod get_server_configs;
mod get_database_configs;
mod get_data_server_configs;

pub use configs::*;
pub use get_language_code::*;
pub use database_configs::*;
pub use server_configs::*;
pub use data_server_configs::*;
pub use tls_configs::*;

pub use get_server_configs::*;
pub use get_database_configs::*;
pub use get_data_server_configs::*;

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
        assert_eq!(configs.server.use_tls, false);
        assert_eq!(configs.server.login_limit, 2);
    }
}
