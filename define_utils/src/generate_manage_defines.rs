use std::io::Write;
use std::path::PathBuf;

use dependencies_sync::bson::{self, Document};
use dependencies_sync::glob;
use dependencies_sync::linked_hash_map::LinkedHashMap;



use manage_define::hard_coded_field_names::{
    DATA_TYPE_FIELD_NAME, ID_FIELD_NAME, NAME_MAP_FIELD_NAME,
};
use manage_define::language_keys::ENGLISH;

use crate::{
    generate_dart_schema_code, get_id, get_name_map, get_schema, get_toml_map,
};

/// 输入定义文件表，输出id的常量定义
pub fn generate_manage_defines(src_dirs: &[&str], target_dir: &str, dart_dir: Option<&str>, dart_package_name: Option<&str>) {
    let manage_ids_path_rust = format!("{}/manage_ids.rs", target_dir);
    let manage_field_ids_path_rust = format!("{}/field_ids.rs", target_dir);

    let mut manage_ids_path_dart = "".to_string();
    let mut manage_field_ids_path_dart = "".to_string();
    if let Some(dart_out) = dart_dir {
        manage_ids_path_dart = format!("{}/ids/manage_ids.dart", dart_out);
        manage_field_ids_path_dart = format!("{}/ids/field_ids.dart", dart_out);
    }

    let mut manage_ids_map: LinkedHashMap<String, String> = LinkedHashMap::new();
    let mut manage_field_ids_map: LinkedHashMap<String, Vec<(String, i32, String)>> =
        LinkedHashMap::new();

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
                    let manage_name = get_name_map(&toml_map)
                        .unwrap()
                        .get_str(ENGLISH)
                        .unwrap()
                        .to_string();
                    let schemas_bson = get_schema::get_schema(&toml_map).unwrap();
                    let schemas: Vec<Document> =
                        bson::from_bson(schemas_bson).expect("转换描写失败");

                    let mut fields: Vec<(String, i32, String)> = Vec::new();
                    for item in schemas.iter() {
                        let name_doc = match item.get_document(NAME_MAP_FIELD_NAME) {
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
                        let s_id = item.get_i32(ID_FIELD_NAME).unwrap();
                        let data_type = item.get_str(DATA_TYPE_FIELD_NAME).unwrap();
                        fields.push((s_name.to_string(), s_id, data_type.to_string()));
                    }

                    manage_field_ids_map.insert(manage_name.clone(), fields);
                    manage_ids_map.insert(manage_name, manage_id.to_string());
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
                "pub const {}_MANAGE_ID: &str = \"{}\"; \n",
                name.to_uppercase(),
                id
            ))
            .expect("写入管理rust文件编码错误");

        // dart
        if dart_dir.is_some() {
            manage_ids_file_dart
                .write_fmt(format_args!(
                    "const String {}_MANAGE_ID =\"{}\"; \n",
                    name.to_uppercase(),
                    id
                ))
                .expect("写入管理dart文件编码错误");
        }
    }

    for (name, fields) in manage_field_ids_map.iter() {
        for (s_name, s_id, _) in fields.iter() {
            manage_field_ids_file_rust
                .write_fmt(format_args!(
                    "pub const {}_{}_FIELD_ID:i32 = {};\n",
                    name.to_uppercase(),
                    s_name.to_uppercase(),
                    s_id
                ))
                .expect("写入属性编码rust文件错误");

            if let Some(_out_dir) = dart_dir {
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

    if let Some(out_dir) = dart_dir {
        println!("create dart schemas");
        for (name, fields) in manage_field_ids_map.iter() {
            println!("create schema: {}", name);
            
            // zh: 生成代码因为数据类型问题不能直接使用，只作为开始模板
            let outdir = format!("{}/cache_schemas/generate", out_dir);
            let out_path = PathBuf::from(out_dir);
            if !out_path.exists(){
                // 创建目录
                std::fs::create_dir_all(out_path).expect("创建目录失败");
            }

            let out_file = format!("{}/{}.dart.gen", outdir, name);
            println!("{}", out_file);

            let mut schema_dart_file =
                std::fs::File::create(out_file).expect("打开属性编号dart文件失败");
            let content = generate_dart_schema_code(name, fields, dart_package_name.unwrap());
            schema_dart_file.write_fmt(format_args!("{}", content));
        }
    }
}
