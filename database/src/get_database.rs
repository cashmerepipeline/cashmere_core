use crate::{get_mongodb_client, DatabaseConfigs};

use dependencies_sync::mongodb::Database;
use std::sync::Arc;

static mut CASHMERE_DATABASE: Option<Arc<Database>> = None;

/// 根据设置文件，取得数据库
pub async fn get_database() -> &'static Database {
    unsafe {
        if CASHMERE_DATABASE.is_some() {
            CASHMERE_DATABASE.as_ref().unwrap()
        } else {
            let database_configs = configs::get_config::<DatabaseConfigs>().unwrap();
            let client = get_mongodb_client().await;

            CASHMERE_DATABASE
                .get_or_insert_with(|| Arc::new(client.database(database_configs.name.as_str())));

            CASHMERE_DATABASE.as_ref().unwrap()
        }
    }
}
