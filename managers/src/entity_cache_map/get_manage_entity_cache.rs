use std::collections::BTreeMap;
use std::sync::Arc;

use dependencies_sync::bson::{doc, Document};
use dependencies_sync::futures::StreamExt;
use dependencies_sync::parking_lot::RwLock;
use manage_define::general_field_ids::ID_FIELD_ID;

use crate::entity_cache_map::get_entity_cache_map;

pub async fn get_manage_entity_cache(
    manage_id: &'static str,
) -> Arc<RwLock<BTreeMap<String, Arc<Document>>>> {
    let c_map_arc = get_entity_cache_map();
    let e_map = {
        let c_map = c_map_arc.read();
        c_map.get(manage_id).cloned()
    };

    if let Some(r) = e_map {
        return r.clone();
    }

    // 不存在则初始化缓存
    let new_map = Arc::new(RwLock::new(BTreeMap::new()));
    {
        let mut cursor = entity::get_query_cursor(&manage_id, doc! {}, &[], None, None, 0).await;

        if let Ok(mut r) = cursor {
            while let Some(d) = r.next().await {
                if let Ok(d) = d {
                    let id = d.get_str(ID_FIELD_ID.to_string()).unwrap().to_string();
                    let mut m = new_map.write();
                    m.insert(id, Arc::new(d));
                    // println!("{:?}", d);
                }
            }
        }
    }
    {
        let mut c_map = c_map_arc.write();
        c_map.insert(manage_id, new_map.clone());
    }

    new_map.clone()
}
