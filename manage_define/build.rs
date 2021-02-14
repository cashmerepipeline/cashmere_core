use glob::glob;

use linked_hash_map::LinkedHashMap;
use bson;
use bson::{Document, Bson};
use toml;

use std::{path::Path};
use std::io::prelude::*;
use std::io::Error;
use std::fs::File;
use std::{fmt::{Display, Formatter}};
use toml::Value;
use toml::map::Map;
use serde::{Serialize, Deserialize};

// use define_utils::{get_toml_map, get_name, get_id, get_schema};

// 拷贝自cashcore-field
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum FieldDataType {
    Bool,
    Int32,
    Int64,
    UInt32,
    UInt64,
    F32,
    F64,
    String,
    List,
    Bytes,
    Map,
    Date,
    DateTime,
    None,
}

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum Language {
    Cn,
    En,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Name {
    language: Language,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PropertyField {
    pub id: i32,
    pub name: LinkedHashMap<String, String>,
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

impl From<String> for FieldDataType {
    fn from(s: String) -> Self {
        if s == "Bool".to_string() { return FieldDataType::Bool; } else if s == "Int32".to_string() { return FieldDataType::Bool; } else if s == "Int64".to_string() { return FieldDataType::Bool; } else if s == "UInt32".to_string() { return FieldDataType::Bool; } else if s == "UInt64".to_string() { return FieldDataType::Bool; } else if s == "F32".to_string() { return FieldDataType::Bool; } else if s == "F64".to_string() { return FieldDataType::Bool; } else if s == "String".to_string() { return FieldDataType::Bool; } else if s == "List".to_string() { return FieldDataType::Bool; } else if s == "Bytes".to_string() { return FieldDataType::Bool; } else if s == "Map".to_string() { return FieldDataType::Bool; } else if s == "Date".to_string() { return FieldDataType::Bool; } else if s == "DateTime".to_string() { return FieldDataType::Bool; } else { return FieldDataType::None; }
    }
}


impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Cn => write!(f, "Cn"),
            Language::En => write!(f, "En"),
        }
    }
}

impl PropertyField {
    pub fn has_name(&self, name: &String) -> bool {
        let names: Vec<String> =
            self.name.values()
                .map(|v| v.clone())
                .collect();

        names.contains(name)
    }

    pub fn from_toml(toml: &toml::map::Map<String, toml::Value>) -> PropertyField {
        let id = toml.get("id").unwrap().as_integer().unwrap() as i32;
        let name: LinkedHashMap<String, String> = toml::from_str(&toml.get("name").unwrap().to_string()).unwrap();
        // println!("{:?}", name);
        let data_type: FieldDataType = toml::from_str(&toml.get("data_type").unwrap().to_string()).unwrap();
        let removed: bool = toml.get("removed").unwrap().as_bool().unwrap();

        PropertyField {
            id: id,
            name: name,
            data_type: data_type,
            removed: removed,
        }
    }

    pub fn from_bson(doc: &bson::Document) -> PropertyField {
        let id = doc.get_i32("id").unwrap();
        let name: LinkedHashMap<String, String> = bson::from_document(doc.get_document("name").unwrap().clone()).unwrap();
        // println!("{:?}", name);
        let data_type: FieldDataType = bson::from_bson(doc.get("data_type").unwrap().clone()).unwrap();
        let removed: bool = doc.get_bool("removed").unwrap();

        PropertyField {
            id: id,
            name: name,
            data_type: data_type,
            removed: removed,
        }
    }
}

/// 读取文件为toml_map
pub fn get_toml_map(toml_path: &String) -> Option<toml::map::Map<String, toml::Value>> {
    let mut toml_file =
        match std::fs::File::open(toml_path) {
            Ok(r) => r,
            Err(_) => {
                println!("初始化数据库文件不存在: {}", toml_path);
                return None;
            }
        };

    let mut toml_string = "".to_string();
    match toml_file
        .read_to_string(&mut toml_string)
    {
        Ok(_) => {}
        Err(_) => {
            println!("读取文件错误：{}", toml_path);
            return None;
        }
    }

    let toml_map: toml::map::Map<String, toml::Value> =
        match toml::from_str(&toml_string) {
            Ok(r) => r,
            Err(_e) => {
                println!("管理定义文件定义错误: {}", toml_path);
                return None;
            }
        };

    Some(toml_map)
}

// 拷贝自 utils
/// 取得管理id
pub fn get_id(toml_map: &toml::map::Map<String, toml::Value>) -> Option<i32> {
    let result = toml_map.get("id");
    match result {
        Some(v) => Some(v.as_integer().unwrap() as i32),
        None => None,
    }
}

/// 取得管理名
pub fn get_name(toml_map: &toml::map::Map<String, toml::Value>) -> Option<Document> {
    let value = toml_map.get("name").expect("取得管理名数据失败");

    let name_map: LinkedHashMap<String, String> =
        toml::from_str(&value.to_string()).expect("建立管理名数据表失败");

    match bson::to_document(&name_map) {
        Ok(r) => Some(r),
        Err(_e) => None,
    }
}

/// 管理描写
pub fn get_schema(toml_map: &toml::map::Map<String, toml::Value>) -> Option<Bson> {
    let id = get_id(toml_map).unwrap();
    let value = toml_map.get("schema").expect("取得描写数据失败");
    let mut schema_vec: Vec<Document> = Vec::new();
    for v in value.as_array().unwrap().iter() {
        let field_toml = match v.as_table() {
            None => {
                println!("错误 {}-{}", id, v.to_string());
                panic!("定义文件错误")
            }
            Some(r) => r
        };
        let field: PropertyField = PropertyField::from_toml(field_toml);
        // println!("{:?}", field);
        let mut temp_doc = Document::new();
        temp_doc.insert("id", field.id);
        temp_doc.insert("data_type", field.data_type.to_string());
        temp_doc.insert("name", bson::to_document(&field.name).unwrap());
        temp_doc.insert("removed", &field.removed);

        schema_vec.push(temp_doc);
    }
    // toml::from_str(&value.to_string()).expect("转换描写列表失败");

    match bson::to_bson(&schema_vec) {
        Ok(r) => Some(r),
        Err(_e) => {
            println!("转换描写失败");
            None
        }
    }
}


fn main() {
    let manage_ids_path = "../manage_define/src/manage_ids.rs";
    let manage_field_ids_path = "../manage_define/src/field_ids.rs";

    let mut manage_ids_map: LinkedHashMap<String, i32> = LinkedHashMap::new();
    let mut manage_field_ids_map: LinkedHashMap<String, Vec<(String, i32)>> = LinkedHashMap::new();

    for entry in glob("../manage_define/defines/**/*.toml").expect("读取tmol文件失败") {
        match entry {
            Ok(path) => {
                let toml_path = path.to_str().unwrap();
                let toml_map =
                    match get_toml_map(&toml_path.to_string()) {
                        Some(r) => r,
                        None => continue
                    };
                let manage_id = match get_id(&toml_map) {
                    Some(r) => r,
                    None => {
                        println!("取得id错误 {}", toml_path);
                        continue;
                    }
                };
                let manage_name = get_name(&toml_map).unwrap().get_str("En").unwrap().to_string();
                let schemas_bson = get_schema(&toml_map).unwrap();
                let schemas: Vec<Document> = bson::from_bson(schemas_bson).expect("转换描写失败");

                let mut fields: Vec<(String, i32)> = Vec::new();
                for item in schemas.iter() {
                    let name_doc =
                        match item.get_document("name") {
                            Ok(r) => r,
                            Err(_) => {
                                panic!("管理定义错误 {} ", manage_name);
                            }
                        };
                    let s_name = match name_doc.get_str("En") {
                        Ok(r) => r,
                        Err(_) => {
                            panic!("管理定义错误 {}", name_doc);
                        }
                    };
                    let s_id = item.get_i32("id").unwrap();
                    fields.push((s_name.to_string(), s_id));
                }
                manage_field_ids_map.insert(manage_name.clone(), fields);
                manage_ids_map.insert(manage_name, manage_id);
            }
            Err(e) => println!("{:?}", e),
        }
    }

    let mut manage_ids_file =
        std::fs::File::create(manage_ids_path).expect("打开管理编号文件失败");
    let mut manage_field_ids_file =
        std::fs::File::create(manage_field_ids_path).expect("打开属性编号文件失败");

    for (name, id) in manage_ids_map.iter() {
        manage_ids_file.write_fmt(format_args!("pub const {}_MANAGE_ID:i32 = {}; \n", name.to_uppercase(), id)).expect("写入管理文件编码错误");
    }

    for (name, fields) in manage_field_ids_map.iter() {
        for (s_name, s_id) in fields.iter() {
            manage_field_ids_file.write_fmt(format_args!("pub const {}_{}_FIELD_ID:i32 = {};\n", name.to_uppercase(), s_name.to_uppercase(), s_id)).expect("写入属性编码文件错误")
        }
    }
}


