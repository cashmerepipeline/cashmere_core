use dependencies_sync::bson::{doc, Document};
use manage_define::general_field_ids::{MODIFIER_FIELD_ID, MODIFY_TIMESTAMP_FIELD_ID};

pub fn add_modify_update_fields(account_id: &str, modify_doc: &mut Document) -> Document {
    // 修改人
    let update_modifier_doc = doc! {MODIFIER_FIELD_ID.to_string():account_id};
    if modify_doc.contains_key("$set") {
        let set_doc = modify_doc.get_document_mut("$set").unwrap();
        set_doc.insert(MODIFIER_FIELD_ID.to_string(), account_id);
    } else {
        modify_doc.insert("$set".to_string(), update_modifier_doc);
    }

    // 时间戳
    if modify_doc.contains_key("$currentDate") {
        let current_date_doc = modify_doc.get_document_mut("$currentDate").unwrap();
        current_date_doc.insert(
            MODIFY_TIMESTAMP_FIELD_ID.to_string(),
            doc! {"$type":"timestamp"},
        );
    } else {
        modify_doc.insert(
            "$currentDate",
            doc! { MODIFY_TIMESTAMP_FIELD_ID.to_string(): {"$type":"timestamp"}},
        );
    }

    modify_doc.clone()
}
