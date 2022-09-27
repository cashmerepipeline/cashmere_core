use bson::{doc, Document};
use property_field::PropertyField;
use tokio_stream::StreamExt;

use manage_define::cashmere::*;

use crate::can_field_read;

pub async fn get_manage_schema_view(
    account_id: &String,
    group: &String,
    manage_id: &String,
    fields: &Vec<PropertyField>,
) -> Document {
    let mut field_stream = tokio_stream::iter(fields);

    // 可见性过滤
    let mut props = doc! {};
    while let Some(field) = field_stream.next().await {
        if can_field_read(account_id, group, manage_id, &field.id.to_string()).await {
            props.insert(field.id.to_string(), 1);
        } else {
            props.insert(field.id.to_string(), 0);
        }
    }

    props
}
