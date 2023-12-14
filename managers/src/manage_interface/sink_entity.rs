use cash_result::{add_call_name_to_chain, OperationResult};
use dependencies_sync::bson::Document;
use manage_define::general_field_ids::ID_FIELD_ID;

use crate::entity_cache_map::cache_update_entity;

pub async fn sink_entity(
    manage_id: &'static str,
    new_entity_doc: &mut Document,
    account_id: &str,
    group_id: &str,
    has_cache: bool,
) -> Result<String, OperationResult> {
    let entity_id = new_entity_doc
        .get_str(ID_FIELD_ID.to_string())
        .unwrap()
        .to_owned();

    // zh: 先更新缓存
    // zh: 如果有缓存则更新缓存
    let old_doc = if has_cache {
        cache_update_entity(manage_id, entity_id.as_str(), new_entity_doc.clone())
    } else {
        None
    };

    // zh: 再添加到数据库
    let result = match entity::insert_entity(
        manage_id.to_string().as_str(),
        new_entity_doc,
        account_id,
        group_id,
    )
    .await
    {
        Ok(r) => Ok(r),
        Err(e) => {
            if has_cache && old_doc.is_some() {
                cache_update_entity(manage_id, entity_id.as_str(), old_doc.unwrap());
            }
            Err(add_call_name_to_chain(e, "sink_entity".to_string()))
        }
    };

    result
}
