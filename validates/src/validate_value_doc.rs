use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::bson;
use dependencies_sync::bson::Document;
use dependencies_sync::tonic::Status;
use dependencies_sync::log;

use cash_core::SchemaField;

pub fn validate_value_doc(
    new_value: &[u8],
    manage_id: &str,
    field_id: &String,
    fields: Vec<SchemaField>,
) -> Result<(), Status> {
    let new_value_doc: Document = if let Ok(r) = bson::from_slice(new_value) {
        r
    } else {
        return Err(Status::invalid_argument(format!(
            "{}:  {}-{}, {}",
            t!("新值反序列化失败"),
            manage_id,
            field_id,
            "validate_value_doc"
        )));
    };
    log::debug!("{}: {:?}", t!("新值文档为"), new_value_doc);

    if new_value_doc.keys().count() != 1 {
        return Err(Status::invalid_argument(format!(
            "{}:{}-{}, {}",
            t!("新值只能有一个字段"),
            manage_id,
            field_id,
            "validate_value_doc"
        )));
    }

    // 验证新值
    // 只能有一个field
    // 新值field必须与目标field一致
    let new_field = new_value_doc
        .keys()
        .next()
        .expect(t!("取得新值字段失败").as_str());
    if field_id != new_field {
        log::error!("{}: {}-{}", t!("新值字段需要与目标字段一致"), manage_id, field_id);
        return Err(Status::invalid_argument(format!(
            "{}:  {}-{}, {}",
            t!("新值字段必须与目标字段一致"),
            manage_id,
            field_id,
            "validate_value_doc"
        )));
    }
    let new_value_bson = new_value_doc.get(field_id).unwrap();
    
    let new_t = new_value_bson.element_type();
    let f_t = fields
        .iter()
        .find(|f| &f.id.to_string() == field_id)
        .unwrap()
        .get_element_type();
    
    log::debug!("{}: {:?}, {:?}", t!("字段类型分别为"), f_t, new_t);

    // field类型需要匹配
    if new_t != f_t {
        log::error!("{}: {}-{}", t!("新值字段类型需要与目标字段类型一致"), manage_id, field_id);
        return Err(Status::invalid_argument(format!(
            "{}:  {}-{}, {}",
            t!("新值字段类型需要与目标字段类型一致"),
            manage_id,
            field_id,
            "validate_value_doc"
        )));
    }

    Ok(())
}
