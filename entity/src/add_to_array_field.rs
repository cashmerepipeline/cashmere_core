use cash_result::*;
use dependencies_sync::bson::doc;
use dependencies_sync::log::trace;
use dependencies_sync::mongodb::bson::Document;
use dependencies_sync::rust_i18n::{self, t};

use crate::utils::add_modify_update_fields;

///  添加实体到数组字段
/// 数组相当于集合，不会重复添加
pub async fn add_to_array_field(
    manage_id: &str,
    query_doc: Document,
    modify_doc: Document,
    account_id: &str,
) -> Result<OperationResult, OperationResult> {
    // 集合是否存在， 不自动创建集合
    let collection = match database::get_collection_by_id(manage_id).await {
        Some(c) => c,
        None => return Err(collection_not_exists("push_entity_array_field")),
    };

    if !modify_doc.contains_key("$addToSet") {
        return Err(operation_failed(
            "add_to_array_field",
            format!("{}: {}", t!("需要包含$addToSet"), modify_doc)),
        );
    }

    let mut _modify_doc = doc! { "$addToSet": modify_doc.clone()};

    let _modify_doc = add_modify_update_fields(account_id, &mut _modify_doc.clone());

    // 更新
    let result = collection
        .update_one(query_doc.clone(), _modify_doc, None)
        .await;

    // 结果
    match result {
        Ok(r) => match r.modified_count {
            0 => {
                trace!("没有实体被更新: {}-{}", query_doc, modify_doc);
                Ok(operation_succeed("succeed"))
            }
            1 => Ok(operation_succeed("succeed")),
            _ => Err(operation_failed(
                "push_entity_array_field",
                format!("更新了多个实体{}-{}", query_doc, r.modified_count),
            )),
        },
        Err(_e) => Err(operation_failed(
            "push_entity_array_field",
            format!("添加操作失败{}", query_doc),
        )),
    }
}
