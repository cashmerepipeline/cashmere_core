use mongodb::Database;
use toml::map::Map;
use toml::Value;

use define_utils as utils;

pub async fn init_event_types(db: &Database, tomls: &Vec<Map<String, Value>>) {
    for map in tomls {
        let queues = match utils::get_queues(map) {
            Some(q) => q,
            None => continue,
        };
        for q in queues {
            db.create_collection(q.as_str(), None)
                .await
                .expect("创建管理集合失败");
        }
    }
}
