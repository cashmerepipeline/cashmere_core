use bson::{doc, Document};
use cash_result::{collection_not_exists, operation_failed, OperationResult};
use manage_define::manage_ids::VIEW_RULES_MANAGE_ID;

/// 取得管理映射规则
pub async fn get_manage_view_rules_doc(manage_name: &String) -> Result<Document, OperationResult> {
    //1. 取得映像记录
    let view_collection =
        match database::get_collection_by_id(&VIEW_RULES_MANAGE_ID.to_string()).await {
            Some(r) => r,
            None => return Err(collection_not_exists("get_manage_view_rules_doc")),
        };
    let view_rules_doc = if let Ok(result) = view_collection
        .find_one(
            doc! {
                "name": manage_name.clone()
            },
            None,
        )
        .await
    {
        if let Some(r) = result {
            r
        } else {
            return Err(operation_failed(
                "get_manage_view_rules_doc",
                "映射规则不存在",
            ));
        }
    } else {
        return Err(operation_failed(
            "get_manage_view_rules_doc",
            "取得映射规则失败",
        ));
    };

    Ok(view_rules_doc)
}
