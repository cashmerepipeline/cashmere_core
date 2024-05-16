use std::sync::Arc;

use dependencies_sync::bson::{doc};
use dependencies_sync::indexmap::IndexMap;
use dependencies_sync::log::info;
use dependencies_sync::parking_lot::RwLock;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tokio_stream::StreamExt;


use manage_define::general_field_ids::ID_FIELD_ID;



use super::MEntityCacheMap;

pub async fn cache_init_cache(manage_id: &'static str) -> Option<MEntityCacheMap> {
    let new_map = MEntityCacheMap::new(RwLock::new(IndexMap::new()));

    {
        let cursor = entity::get_query_cursor(manage_id, doc! {}, &[], None, None, 0).await;

        if let Ok(mut r) = cursor {
            while let Some(d) = r.next().await {
                if let Ok(d) = d {
                    let id = d.get_str(ID_FIELD_ID.to_string()).unwrap().to_string();
                    let mut m = new_map.write();
                    m.insert(id, Arc::new(RwLock::new(d)));
                    // println!("{:?}", d);
                }
            }
        }
    }

    info!(
        "{}: {}",
        t!("初始化缓存完成"),
        manage_id,
    );

    Some(new_map)
}
