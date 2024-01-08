use dependencies_sync::bson::doc;
use dependencies_sync::log::info;
use dependencies_sync::rust_i18n::{self, t};

use cash_result::{operation_failed, operation_succeed, OperationResult};
use crate::get_ids_collection;

/// 初始化实体编号字段
pub async fn init_ids_count_field(manage_id: &str) -> Result<OperationResult, OperationResult> {
    let ids_collection = get_ids_collection().await;

    let f_result = ids_collection
        .find_one(
            doc! {
                "_id": manage_id.clone()
            },
            None,
        )
        .await;

    if let Ok(r) = f_result {
        if r.is_none() {
            match ids_collection
                .insert_one(
                    doc! {
                        "_id": manage_id.clone(),
                        "id_count": 0i64
                    },
                    None,
                )
                .await
            {
                Ok(_r) => (),
                Err(_e) => {
                    return Err(operation_failed(
                        "init_ids_count_field",
                        "初始化序列号记录失败",
                    ))
                }
            };
        } else {
            info! ("{}: {}", t!("序号生成器记录已存在"), manage_id);
            return Ok(operation_succeed("already exits"));
        }
    } else {
        return Err(operation_failed(
            "init_ids_count_field",
            "初始化序列号记录失败",
        ));
    }

    Ok(operation_succeed("ok"))
}
