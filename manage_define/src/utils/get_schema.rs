use dependencies_sync::bson::{self, Bson, Document};
use dependencies_sync::log::error;
use dependencies_sync::toml;

use property_field::PropertyField;

use crate::utils::get_id;

/**
 * 取得管理数据类型
 * @param toml_map 定义toml数据
 * @return 管理字段列表bson
 */
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

        let field: PropertyField = PropertyField::from_toml(field_toml, &(field_id as i32));
        // println!("{:?}", field);

        let mut temp_doc = Document::new();
        temp_doc.insert("id", (field_id + 1 + 2000) as i32);
        temp_doc.insert("data_type", field.data_type.to_string());
        temp_doc.insert("name", bson::to_document(&field.name_map).unwrap());
        temp_doc.insert("removed", field.removed);

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
