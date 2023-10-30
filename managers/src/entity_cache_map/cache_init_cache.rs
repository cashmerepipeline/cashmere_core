use std::sync::Arc;

use dependencies_sync::futures::StreamExt;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::bson::doc;
use dependencies_sync::log::info;

use cash_result::{operation_failed, operation_succeed, OperationResult};
use manage_define::general_field_ids::ID_FIELD_ID;

use crate::entity_cache_map::get_manage_entity_cache;

pub async fn cache_init_cache(
    manage_id: i32,
) -> Result<OperationResult, OperationResult> {
    let mut entities = vec![];
    let mut cursor = if let Ok(r) =
        entity::get_query_cursor(&manage_id.to_string(), doc! {}, None, None).await
    {
        r
    } else {
        return Err(operation_failed("init_cache", "初始化缓存失败".to_string()));
    };

    while let Some(doc_result) = cursor.next().await {
        if let Ok(doc) = doc_result {
            entities.push(doc);
        }
    }

    {
        let m_cahce_map_arc = get_manage_entity_cache(manage_id);
        let mut cache_map = m_cahce_map_arc.write();
        for entity in entities {
            let id = entity.get_str(ID_FIELD_ID.to_string()).unwrap().to_string();
            cache_map.insert(id, Arc::new(entity));
        }

        info!( "{}: {}, {}", t!("初始化缓存完成"), manage_id, cache_map.len() );
    }

    Ok(operation_succeed("ok"))
}