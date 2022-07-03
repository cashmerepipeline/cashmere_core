/*
Author: 闫刚 (yes7rose@sina.com)
field.rs (c) 2020
Desc: 属性
Created:  2020-11-26T12:47:25.692Z
Modified: !date!
*/

use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use linked_hash_map::LinkedHashMap;

use crate::field_data_type::FieldDataType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PropertyField {
    pub id: i32,
    pub name_map: LinkedHashMap<String, String>,
    pub data_type: FieldDataType,
    pub removed: bool,
}

impl Display for FieldDataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldDataType::Bool => write!(f, "Bool"),
            FieldDataType::Int32 => write!(f, "Int32"),
            FieldDataType::Int64 => write!(f, "Int64"),
            FieldDataType::UInt32 => write!(f, "UInt32"),
            FieldDataType::UInt64 => write!(f, "UInt64"),
            FieldDataType::F32 => write!(f, "F32"),
            FieldDataType::F64 => write!(f, "F64"),
            FieldDataType::Range => write!(f, "Range"),
            FieldDataType::String => write!(f, "String"),
            FieldDataType::List => write!(f, "List"),
            FieldDataType::Bytes => write!(f, "Bytes"),
            FieldDataType::Map => write!(f, "Map"),
            FieldDataType::Date => write!(f, "Date"),
            FieldDataType::DateTime => write!(f, "DateTime"),
            FieldDataType::None => write!(f, "None"),
        }
    }
}

impl PropertyField {
    pub fn has_name(&self, name: &String) -> bool {
        self.name_map.values().cloned().any(|x| x == *name)
    }

    pub fn from_toml(toml: &toml::map::Map<String, toml::Value>, id: &i32) -> PropertyField {
        let name_map: LinkedHashMap<String, String> =
            toml::from_str(&toml.get("name_map").unwrap().to_string()).unwrap();
        println!("{:?}", name_map);
        let data_type: FieldDataType =
            toml::from_str(&toml.get("data_type").unwrap().to_string()).unwrap();
        let removed: bool = toml.get("removed").unwrap().as_bool().unwrap();

        PropertyField {
            id: *id,
            name_map: name_map,
            data_type,
            removed,
        }
    }

    pub fn from_bson(doc: &bson::Document) -> PropertyField {
        let id = doc.get_i32("id").unwrap();
        let name: LinkedHashMap<String, String> =
            bson::from_document(doc.get_document("name").unwrap().clone()).unwrap();
        // println!("{:?}", name);
        let data_type: FieldDataType =
            bson::from_bson(doc.get("data_type").unwrap().clone()).unwrap();
        let removed: bool = doc.get_bool("removed").unwrap();

        PropertyField {
            id,
            name_map: name,
            data_type,
            removed,
        }
    }
}
