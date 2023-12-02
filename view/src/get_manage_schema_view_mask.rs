use std::collections::HashMap;

use  dependencies_sync::bson::{doc, Document};
use cash_core::SchemaField;

use crate::can_field_read;

/// 获取字段是否可读，1为可读，0为不可读
pub async fn get_manage_schema_view_mask(
    manage_id: &i32,
    fields: &Vec<SchemaField>,
    role_group: &String,
) -> HashMap<String, bool> {
    let field_stream = fields.iter();

    // 可见性过滤
    let mut props = HashMap::new();
    for field in field_stream {
        if can_field_read(manage_id, &field.id.to_string(), role_group).await {
            props.insert(field.id.to_string(), true);
        } else {
            props.insert(field.id.to_string(), false);
        }
    }

    props
}
