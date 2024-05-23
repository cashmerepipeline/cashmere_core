use dependencies_sync::bson::{self, Bson, Document};
use dependencies_sync::log::error;
use dependencies_sync::toml;

use manage_define::hard_coded_field_names::{
    DATA_TYPE_FIELD_NAME, ID_FIELD_NAME, NAME_MAP_FIELD_NAME, REMOVED_FIELD_NAME,
};
use cash_core::SchemaField;

use crate::get_id;

/// 管理描写
pub fn get_schema(toml_map: &toml::map::Map<String, toml::Value>) -> Option<Bson> {
    let manage_id = get_id::get_id(toml_map).unwrap();

    let value = toml_map.get("schema").expect("取得描写数据失败");
    let mut schema_vec: Vec<Document> = Vec::new();
    for (field_id, v) in value.as_array().unwrap().iter().enumerate() {
        let field_toml = match v.as_table() {
            None => {
                error!("错误 {}-{}", manage_id, v.to_string());
                panic!("定义文件错误")
            }
            Some(r) => r,
        };
        let field: SchemaField =
            SchemaField::from_toml(field_toml, &((field_id + 2001) as i32));
        // println!("{:?}", field);
        let mut temp_doc = Document::new();
        temp_doc.insert(ID_FIELD_NAME, field.id);
        temp_doc.insert(DATA_TYPE_FIELD_NAME, field.data_type.to_string());
        temp_doc.insert(
            NAME_MAP_FIELD_NAME,
            bson::to_document(&field.name_map).unwrap(),
        );
        temp_doc.insert(REMOVED_FIELD_NAME, field.removed);

        schema_vec.push(temp_doc);
    }

    match bson::to_bson(&schema_vec) {
        Ok(r) => Some(r),
        Err(_e) => {
            error!("转换描写失败");
            None
        }
    }
}
