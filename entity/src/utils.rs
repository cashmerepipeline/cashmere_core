use mongodb::{Collection, bson::doc};
use chrono::Utc;

use cash_core::field::ids::*;

pub(crate) async fn update_entity_modify_stamp(
    entity_id: &String,
    collection: Collection,
    account_id: &String,
) -> bool {
    // 记录修改
    match collection
        .update_one(
            doc! {
                "_id": entity_id
            },
            doc! {
                "$set": {
                    MODIFIER_FIELD_ID.to_string(): account_id.clone(),
                    MODIFY_TIMESTAMP_FIELD_ID.to_string(): Utc::now().timestamp()
                }
            },
            None,
        )
        .await {
        Ok(_r) => true,
        _ => false
    }
}