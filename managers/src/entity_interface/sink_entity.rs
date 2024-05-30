use dependencies_sync::bson::Document;


use cash_result::{add_call_name_to_chain, OperationResult};
use manage_define::general_field_ids::ID_FIELD_ID;

pub async fn sink_entity(
    manage_id: &'static str,
    new_entity_doc: &mut Document,
    account_id: &str,
    group_id: &str,
) -> Result<String, OperationResult> {
    let _entity_id = new_entity_doc
        .get_str(ID_FIELD_ID.to_string())
        .unwrap()
        .to_owned();

    match entity::insert_entity(
        manage_id.to_string().as_str(),
        new_entity_doc,
        account_id,
        group_id,
    )
    .await
    {
        Ok(r) => Ok(r),
        Err(e) => {
            // zh: 更新数据库失败则恢复缓存
            // if let Some(old_doc) = old_doc {
                // if has_cache {
                    // cache_update_entity(manage_id, entity_id.as_str(), old_doc).await;
                // }
            // }
            Err(add_call_name_to_chain(e, "sink_entity".to_string()))
        }
    }
}
