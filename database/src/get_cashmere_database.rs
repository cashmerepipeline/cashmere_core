use crate::DatabaseConfigs;
use dependencies_sync::mongodb::{Client, Database};
use dependencies_sync::mongodb::options::{ClientOptions, ServerAddress};
use std::sync::Arc;

static mut CASHMERE_DATABASE: Option<Arc<Database>> = None;

/// 根据设置文件，取得数据库
pub async fn get_cashmere_database() -> &'static Database {
    unsafe {
        if CASHMERE_DATABASE.is_some() {
            CASHMERE_DATABASE.as_ref().unwrap()
        } else {
            let database_configs = configs::get_config::<DatabaseConfigs>().unwrap();

            CASHMERE_DATABASE.get_or_insert_with(|| {
                let options = ClientOptions::builder()
                    .hosts(vec![ServerAddress::Tcp {
                        host: database_configs.address.clone(),
                        port: Some(database_configs.port),
                    }])
                    .build();

                let client = match Client::with_options(options) {
                    Ok(r) => r,
                    Err(_e) => panic!("连接到数据库失败"),
                };
                Arc::new(client.database(database_configs.name.as_str()))
            });

            CASHMERE_DATABASE.as_ref().unwrap()
        }
    }
}
