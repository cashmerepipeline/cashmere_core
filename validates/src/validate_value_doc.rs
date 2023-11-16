use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::bson;
use dependencies_sync::bson::Document;
use dependencies_sync::tonic::Status;

use cash_core::PropertyField;

pub fn validate_value_doc(
    new_value: &[u8],
    field_id: &String,
    fields: Vec<PropertyField>,
) -> Result<(), Status> {
    let new_value_doc: Document = if let Ok(r) = bson::from_slice(new_value) {
        r
    } else {
        return Err(Status::invalid_argument(format!(
            "{}:  {}",
            t!("新值反序列化失败"),
            "edit_entity_field"
        )));
    };
    if new_value_doc.keys().count() != 1 {
        return Err(Status::invalid_argument(format!(
            "{}:  {}",
            t!("新值只能有一个field"),
            "edit_entity_field"
        )));
    }
    let new_field = new_value_doc
        .keys()
        .next()
        .expect(t!("取得新值field失败").as_str());
    if field_id != new_field {
        return Err(Status::invalid_argument(format!(
            "{}:  {}",
            t!("新值field必须与目标field一致"),
            "edit_entity_field"
        )));
    }
    let new_value_bson = new_value_doc.get(field_id).unwrap();
    let new_t = new_value_bson.element_type();
    let f_t = fields
        .iter()
        .find(|f| &f.id.to_string() == field_id)
        .unwrap()
        .get_element_type();
    // 验证新值
    // 只能有一个field
    // 新值field必须与目标field一致

    // field类型需要匹配
    if new_t != f_t {
        return Err(Status::invalid_argument(format!(
            "{}:  {}",
            t!("新值field类型需要与目标field类型一致"),
            "edit_entity_field"
        )));
    }

    Ok(())
}
