use dependencies_sync::bson::{doc, Document};
use manage_define::general_field_ids::MODIFY_TIMESTAMP_FIELD_ID;

pub fn get_timestamp_update_doc() -> Document {
    doc!(
      "$currentDate": {
        MODIFY_TIMESTAMP_FIELD_ID.to_string(): { "$type": "timestamp" }
     }
    )
}
