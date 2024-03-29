use dependencies_sync::bson::{self, doc};
use dependencies_sync::tokio_stream::{self as stream, StreamExt};

use crate::can_field_read;

pub async fn filter_can_read_fields(
    result: &bson::Document,
    manage_id: &i32,
    role_group: &String,
) -> bson::Document {
    let mut result_doc = doc!();
    let mut property_stream = stream::iter(result);

    while let Some((k, v)) = property_stream.next().await {
        if !can_field_read(&manage_id.to_string(), &k, role_group).await {
            if k == &"_id".to_string() {
                result_doc.insert(k, v);
            }
            continue;
        }
        result_doc.insert(k, v);
    }
    result_doc
}
