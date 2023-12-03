use crate::DatabaseConfigs;
use configs::ConfigTrait;
use dependencies_sync::mongodb::Client;
use dependencies_sync::mongodb::options::{ClientOptions, ServerAddress};
use std::sync::Arc;

static mut MONGODB_CLIENT: Option<Arc<Client>> = None;

/// 取得客户端
pub async fn get_mongodb_client() -> &'static Client {
    unsafe {
        if MONGODB_CLIENT.is_some() {
            MONGODB_CLIENT.as_ref().unwrap()
        } else {
            let database_configs = DatabaseConfigs::get();

            MONGODB_CLIENT.get_or_insert_with(|| {
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
                Arc::new(client)
            });
            MONGODB_CLIENT.as_ref().unwrap()
        }
    }
}
