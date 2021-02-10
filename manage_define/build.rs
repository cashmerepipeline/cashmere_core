use toml;
use glob::glob;
use bson::{self, Document};

use linked_hash_map::LinkedHashMap;

use minit::define::{get_toml_map, get_name, get_id, get_schema};
use std::io::prelude::*;
use bson::document::ValueAccessError;

fn main() {
    let manage_ids_path = "src/manage_ids.rs";
    let manage_field_ids_path = "src/field_ids.rs";

    let mut manage_ids_map: LinkedHashMap<String, i32> = LinkedHashMap::new();
    let mut manage_field_ids_map: LinkedHashMap<String, Vec<(String, i32)>> = LinkedHashMap::new();

    for entry in glob("defines/**/*.toml").expect("Failed to read glob pattern") {
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
        manage_ids_file.write_fmt(format_args!("const {}_MANAGE_ID:i32 = {}; \n", name.to_uppercase(), id)).expect("写入管理文件编码错误");
    }

    for (name, fields) in manage_field_ids_map.iter() {
        for (s_name, s_id) in fields.iter() {
            manage_field_ids_file.write_fmt(format_args!("const {}_{}_FIELD_ID:i32 = {};\n", name.to_uppercase(), s_name.to_uppercase(), s_id)).expect("写入属性编码文件错误")
        }
    }
}

