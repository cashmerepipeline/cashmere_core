/*
Project: cashmere_server
Creator: 闫刚
Create time: 2020-10-19 11:16
Introduction:
*/

use cash_core::field::PropertyField;
use linked_hash_map::LinkedHashMap;
use bson;
use bson::{Document, Bson};
use toml;
use cash_core::view_rules::{ViewRule, ViewRules};

use std::{path::Path};
use std::io::prelude::*;

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
    let value = toml_map.get("schema").expect("取得描写数据失败");
    let mut schema_vec: Vec<Document> = Vec::new();
    for v in value.as_array().unwrap().iter(){
        // println!("{}", v.to_string());
        let field: PropertyField = PropertyField::from_toml(&v.as_table().unwrap());
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

/// 管理队列
pub fn get_queues(toml_map: &toml::map::Map<String, toml::Value>) -> Option<Vec<String>> {
    let result = toml_map.get("queues").unwrap().as_table();
    match result {
        Some(r) => {
            let mut queues: Vec<String> = vec![];
            for (_key, value) in r {
                println!("{:?}", value);
                if let Some(r) = value.as_str() {
                    queues.push(r.to_string());
                }
            }
            Some(queues)
        }
        None => None,
    }
}

/// 初始记录
pub fn get_init_items(toml_map: &toml::map::Map<String, toml::Value>) -> Option<Vec<Document>> {
    let result = toml_map.get("items").unwrap().as_table();
    match result {
        Some(r) => {
            let mut items: Vec<Document> = vec![];
            for (_key, value) in r {
                // item is table
                let v_table = value.as_table().unwrap();
                if v_table.is_empty() {
                    continue;
                }
                // 构造Document
                let doc = bson::ser::to_document(v_table).unwrap();
                println!("取得初始化实体 {}", doc);
                items.push(doc);
            }
            Some(items)
        }
        None => None,
    }
}

/// 取得映像定义
pub fn get_init_view_rules(toml_map: &toml::map::Map<String, toml::Value>) -> Option<ViewRules> {
    let result = match toml_map.get("view_rules") {
        Some(r) => r.as_table(),
        None => return None,
    };

    match result {
        Some(r) => {
            let manage_str = r.get("manage").unwrap().to_string();
            let entity_str = r.get("entity").unwrap().to_string();
            let schema_str = r.get("schema").unwrap().to_string();

            let manage_map: LinkedHashMap<String, ViewRule> =
                toml::from_str(manage_str.as_str()).unwrap();
            let entity_map: LinkedHashMap<String, ViewRule> =
                toml::from_str(entity_str.as_str()).unwrap();
            let schema_map: LinkedHashMap<String, LinkedHashMap<String, ViewRule>> =
                toml::from_str(schema_str.as_str()).unwrap();

            Some(ViewRules {
                manage: manage_map,
                entity: entity_map,
                schema: schema_map,
            })
        }
        None => return None,
    }
}


// 取得目录下的toml文件
pub fn get_toml_files_of_dir(
    toml_dir: &String
) -> Option<Vec<String>> {
    let mut toml_pathes: Vec<String> = vec![];
    let toml_dir_path = Path::new(toml_dir);
    if toml_dir_path.exists() && toml_dir_path.is_dir() {
        for entry in toml_dir_path
            .read_dir()
            .expect(format!("{} 目录不存在或者不是目录", toml_dir).as_str())
        {
            if let Ok(ref entry) = entry {
                let path_buf = &entry.path();
                if path_buf.is_dir() {
                    continue;
                }
                // 添加toml文件路径
                let path_string = path_buf.to_str().unwrap().to_string();
                if path_string.ends_with(".toml") {
                    println!("取得管理定义文件 {}", path_string);
                    toml_pathes.push(path_string);
                }
            }
        }
    }

    Some(toml_pathes)
}

// 从文件列表创建toml表
pub fn get_tomls_from_pathes(
    toml_pathes: &Vec<String>
) -> Option<Vec<toml::map::Map<String, toml::Value>>>{
    // 读入所有文件并构造toml映射
    let mut tomls: Vec<toml::map::Map<String, toml::Value>> = vec![];
    for toml_path in toml_pathes {
        let mut toml_file = std::fs::File::open(toml_path).expect("初始化数据库文件不存在");

        let mut toml_string = "".to_string();
        toml_file
            .read_to_string(&mut toml_string)
            .expect("管理定义文件读取错误");

        let toml_map: toml::map::Map<String, toml::Value> =
            toml::from_str(&toml_string).expect("管理定义文件定义错误");
        tomls.push(toml_map);
    }

    Some(tomls)
}

// // 创建新管理实体到数据库, 返回新document
// async pub fn new_manage_entity_to_database(
//     id: i32,
//     name: &Document,
//     schema: &Document,
//     creator_id: &String,
//     groups: &Vec<String>,
// ) -> Result<Document, OperationResult> {
//     // id 字符串
//     let id_str = &id.to_string();

//     let manages_collection = database::get_manages_collection().await;
//     let cashmere_db = database::get_cashmere_database();

//     // 创建管理集合, 集合名使用管理id的字符串形式
//     let manage_collection_exists = database::collection_exists(id_str);
//     println!("{} {}", id, manage_collection_exists);
//     if !manage_collection_exists {
//         let result = cashmere_db.create_colalection(&id_str, None).await;
//         match result {
//             Ok(r) => (),
//             Err(_e) => {
//                 return Err(operation_failed(
//                     "new_manange",
//                     format!("创建管理失败: {}", id_str),
//                 ))
//             }
//         };
//     }

//     // 添加新条目到管理集合, id使用字符串形式
//     let manage_entity_exists = manage_entity_exists(id_str);
//     if !manage_entity_exists {
//         let now = Utc::now().timestamp();
//         let empty_list: Vec<String> = vec![];
//         let new_doc = doc! {
//             "_id": id_str,
//             "name": name,
//             "owner": creator_id,
//             "creator": creator_id,
//             "modifier": creator_id,
//             "groups": groups,
//             "create_timestamp": now,
//             "modify_timestamp": now,
//             "schema": schema,
//             "events": empty_list.clone(),
//             "event_queues": empty_list.clone(),
//             "message_queues": empty_list
//         };

//         let result = manages_collection.insert_one(new_doc.clone(), None).await;
//         match result {
//             Ok(r) => {
//                 return Ok(new_doc);
//             }
//             Err(_e) => {
//                 return Err(operation_failed(
//                     "new_manage_entity_to_database",
//                     "插入管理实体失败",
//                 ))
//             }
//         }
//     } else {
//         return Err(operation_failed(
//             "new_manage_entity_to_database",
//             "管理实体已经存在",
//         ));
//     };
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}