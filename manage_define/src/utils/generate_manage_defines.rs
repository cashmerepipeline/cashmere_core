use std::io::Write;

use dependencies_sync::bson::{self, Document};
use dependencies_sync::linked_hash_map::LinkedHashMap;

use crate::{
    language_keys::ENGLISH,
    utils::{get_id, get_name, get_schema, get_toml_map},
};

pub fn generate_manage_defines(src_dirs: &Vec<&str>, target_dir: &str, dart_dir: Option<&str>) {
    let manage_ids_path_rust = format!("{}/manage_ids.rs", target_dir);
    let manage_field_ids_path_rust = format!("{}/field_ids.rs", target_dir);

    let mut manage_ids_path_dart = format!("{}/manage_ids.dart", target_dir);
    let mut manage_field_ids_path_dart = format!("{}/field_ids.dart", target_dir);
    if let Some(dart_out) = dart_dir {
        manage_ids_path_dart = format!("{}/manage_ids.dart", dart_out);
        manage_field_ids_path_dart = format!("{}/field_ids.dart", dart_out);
    }

    let mut manage_ids_map: LinkedHashMap<String, i32> = LinkedHashMap::new();
    let mut manage_field_ids_map: LinkedHashMap<String, Vec<(String, i32)>> = LinkedHashMap::new();

    for dir in src_dirs.iter() {
        let src_dir = format!("{}/*.toml", dir);

        for entry in glob::glob(src_dir.as_ref()).expect("读取tmol文件失败") {
            // println!("{}", entry.as_ref().unwrap().to_str().unwrap());
            match entry {
                Ok(path) => {
                    let toml_path = path.to_str().unwrap();
                    let toml_map = match get_toml_map::get_toml_map(&toml_path.to_string()) {
                        Some(r) => r,
                        None => continue,
                    };
                    let manage_id = match get_id::get_id(&toml_map) {
                        Some(r) => r,
                        None => {
                            println!("取得id错误 {}", toml_path);
                            continue;
                        }
                    };
                    let manage_name = get_name::get_name_map(&toml_map)
                        .unwrap()
                        .get_str(ENGLISH)
                        .unwrap()
                        .to_string();
                    let schemas_bson = get_schema::get_schema(&toml_map).unwrap();
                    let schemas: Vec<Document> =
                        bson::from_bson(schemas_bson).expect("转换描写失败");

                    let mut fields: Vec<(String, i32)> = Vec::new();
                    for item in schemas.iter() {
                        let name_doc = match item.get_document("name") {
                            Ok(r) => r,
                            Err(_) => {
                                panic!("管理定义错误 {} ", manage_name);
                            }
                        };
                        let s_name = match name_doc.get_str(ENGLISH) {
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
    }

    let mut manage_ids_file_rust =
        std::fs::File::create(manage_ids_path_rust).expect("打开管理编号rust文件失败");
    let mut manage_field_ids_file_rust =
        std::fs::File::create(manage_field_ids_path_rust).expect("打开属性编号rust文件失败");

    let mut manage_ids_file_dart =
        std::fs::File::create(manage_ids_path_dart).expect("打开管理编号dart文件失败");
    let mut manage_field_ids_file_dart =
        std::fs::File::create(manage_field_ids_path_dart).expect("打开属性编号dart文件失败");

    for (name, id) in manage_ids_map.iter() {
        manage_ids_file_rust
            .write_fmt(format_args!(
                "pub const {}_MANAGE_ID:i32 = {}; \n",
                name.to_uppercase(),
                id
            ))
            .expect("写入管理rust文件编码错误");
        manage_ids_file_dart
            .write_fmt(format_args!(
                "const int {}_MANAGE_ID = {}; \n",
                name.to_uppercase(),
                id
            ))
            .expect("写入管理dart文件编码错误");
    }

    for (name, fields) in manage_field_ids_map.iter() {
        for (s_name, s_id) in fields.iter() {
            manage_field_ids_file_rust
                .write_fmt(format_args!(
                    "pub const {}_{}_FIELD_ID:i32 = {};\n",
                    name.to_uppercase(),
                    s_name.to_uppercase(),
                    s_id
                ))
                .expect("写入属性编码rust文件错误");
            manage_field_ids_file_dart
                .write_fmt(format_args!(
                    "const int {}_{}_FIELD_ID = {};\n",
                    name.to_uppercase(),
                    s_name.to_uppercase(),
                    s_id
                ))
                .expect("写入属性编码dart文件错误");
        }
    }
}
