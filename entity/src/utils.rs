use dependencies_sync::chrono::Utc;
use dependencies_sync::mongodb::{bson::doc, bson::Document, Collection};

use manage_define::general_field_ids::*;

pub(crate) async fn update_entity_modify_stamp(
    entity_id: &String,
    collection: Collection<Document>,
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