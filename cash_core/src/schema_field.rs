/*
Author: 闫刚 (yes7rose@sina.com)
field.rs (c) 2020
Desc: 属性
Created:  2020-11-26T12:47:25.692Z
Modified: !date!
*/

use dependencies_sync::bson::spec::ElementType;
use dependencies_sync::bson;
use dependencies_sync::indexmap::IndexMap;
use dependencies_sync::toml;

use serde::{Deserialize, Serialize};

use manage_define::hard_coded_field_names::*;

/// 实体属性
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SchemaField {
    pub id: i32,
    // 使用LinkedHashMap是为了保持字段顺序
    pub name_map: IndexMap<String, String>,
    // 使用bson的数据类型
    pub data_type: String,
    pub removed: bool,
}

impl SchemaField {
    pub fn get_element_type(&self) -> ElementType {
        match self.data_type.as_str() {
            "Double" => ElementType::Double,
            "String" => ElementType::String,
            "Array" => ElementType::Array,
            "Document" => ElementType::EmbeddedDocument,
            "Boolean" => ElementType::Boolean,
            "Null " => ElementType::Null,
            "RegularExpression" => ElementType::RegularExpression,
            "JavaScriptCode" => ElementType::JavaScriptCode,
            "JavaScriptCodeWithScope" => ElementType::JavaScriptCodeWithScope,
            "Int32" => ElementType::Int32,
            "Int64" => ElementType::Int64,
            "Timestamp" => ElementType::Timestamp,
            "Binary" => ElementType::Binary,
            "ObjectId" => ElementType::ObjectId,
            "DateTime" => ElementType::DateTime,
            "Symbol" => ElementType::Symbol,
            "Decimal128" => ElementType::Decimal128,
            "Undefined " => ElementType::Undefined,
            "MaxKey " => ElementType::MaxKey,
            "MinKey " => ElementType::MinKey,
            "DbPointer" => ElementType::DbPointer,
            _ => ElementType::Undefined,
        }
    }
}

impl SchemaField {
    pub fn has_name(&self, name: &String) -> bool {
        self.name_map.values().cloned().any(|x| x == *name)
    }

    pub fn from_toml(toml: &toml::map::Map<String, toml::Value>, id: &i32) -> SchemaField {
        // println!("{:?}", toml);
        let t_map = toml.get(NAME_MAP_FIELD_NAME).unwrap().as_table().unwrap();
        let mut name_map: IndexMap<String, String> = IndexMap::new();
        t_map.iter().for_each(|(k, v)| {
            name_map.insert(k.to_owned(), v.as_str().unwrap().to_owned());
        });

        let data_type: String = toml
            .get(DATA_TYPE_FIELD_NAME)
            .unwrap()
            .as_str()
            .unwrap()
            .to_owned();

        let removed: bool = toml.get(REMOVED_FIELD_NAME).unwrap().as_bool().unwrap();

        SchemaField {
            id: *id,
            name_map,
            data_type,
            removed,
        }
    }

    pub fn from_document(doc: &bson::Document) -> SchemaField {
        let id = doc.get_i32(ID_FIELD_NAME).unwrap();

        let name_map: IndexMap<String, String> =
            bson::from_bson(doc.get(NAME_MAP_FIELD_NAME).unwrap().clone()).unwrap();

        let data_type: String =
            bson::from_bson(doc.get(DATA_TYPE_FIELD_NAME).unwrap().clone()).unwrap();

        let removed: bool = doc.get_bool(REMOVED_FIELD_NAME).unwrap();

        SchemaField {
            id,
            name_map,
            data_type,
            removed,
        }
    }
}
