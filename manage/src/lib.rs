/*
Author: 闫刚 (yes7rose@sina.com)
lib.rs (c) 2020
Desc:  管理的 管理
Created:  2020-10-10T02:13:50.739Z
Modified: !date!
*/

use std::sync::Arc;

use chrono::Utc;
use linked_hash_map::LinkedHashMap;
use log::{error, info, warn};
use mongodb::bson;
use mongodb::bson::{doc, Document};
use cash_result::*;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use tokio::stream::StreamExt;
use futures::executor::block_on;

use  manage_define::manage_ids::MANAGES_MANAGE_ID;
use cash_core::Manage;

use database::MongodbResult;

/// 管理表，因为访问频繁，所以需要缓存，提高速度: {编号：管理}
static mut MANAGES_MAP: Option<Arc<RwLock<LinkedHashMap<i32, Manage>>>> = None;
pub async fn get_manages_map() -> &'static RwLock<LinkedHashMap<i32, Manage>> {
    unsafe {
        // 有数据
        if MANAGES_MAP.is_some() {
            MANAGES_MAP.as_ref().unwrap()
        }
        // 没有数据从数据库读取
        else {
            MANAGES_MAP.get_or_insert(build_map_from_database().await);
            MANAGES_MAP.as_ref().unwrap()
        }
    }
}

/// 从数据库读取管理
async fn build_map_from_database() -> Arc<RwLock<LinkedHashMap<i32, Manage>>> {
    let manages_docs = get_manages_documents().await.unwrap();
    let mut result: LinkedHashMap<i32, Manage> = LinkedHashMap::new();
    for m_doc in manages_docs {
        let id = match entity::get_entity_id(&m_doc) {
            Some(r) => r,
            None => {
                println!("取得管理id错误：{:?}", m_doc);
                continue;
            }
        };
        let manage = bson::from_document(m_doc).unwrap();
        result.insert(id.parse::<i32>().unwrap(), manage);
    }
    Arc::new(RwLock::new(result))
}

/// 创建新管理
pub async fn new_manage(
    id: i32,
    name: &Document,
    schema: &Document,
    creator_id: &String,
    groups: &Vec<String>,
) -> Result<OperationResult, OperationResult> {
    match new_manage_entity_to_database(id.clone(), name, schema, creator_id, groups).await {
        Ok(r) => {
            // 更新全局管理缓存列表
            let mut manages_map = get_manages_map().await.write();
            let manage = bson::from_document(r).unwrap();
            manages_map.insert(id, manage);

            return Ok(OperationResult::Succeed(id.to_string()));
        }
        Err(_e) => {
            return Err(operation_failed(
                "new_manage",
                "插入管理实体失败",
            ))
        }
    }
}

/// 创建新管理实体到数据库, 返回新document
pub async fn new_manage_entity_to_database(
    id: i32,
    name: &Document,
    schema: &Document,
    creator_id: &String,
    groups: &Vec<String>,
) -> Result<Document, OperationResult> {
    // id 字符串
    let id_str = &id.to_string();

    let manages_collection = database::get_manages_collection().await;
    let cashmere_db = database::get_cashmere_database().await;

    // 创建管理集合, 集合名使用管理id的字符串形式
    let manage_collection_exists = database::collection_exists(id_str).await;
    println!("{} {}", id, manage_collection_exists);
    if !manage_collection_exists {
        let result = block_on(cashmere_db.create_collection(&id_str, None));
        match result {
            Ok(r) => (),
            Err(_e) => {
                return Err(operation_failed(
                    "new_manange",
                    format!("创建管理失败: {}", id_str),
                ))
            }
        };
    }

    // 添加新条目到管理集合, id使用字符串形式
    let manage_entity_exists = manage_entity_exists(id_str).await;
    if !manage_entity_exists {
        let now = Utc::now().timestamp();
        let empty_list: Vec<String> = vec![];
        let new_doc = doc! {
            "_id": id_str,
            "name": name,
            "owner": creator_id,
            "creator": creator_id,
            "modifier": creator_id,
            "groups": groups,
            "create_timestamp": now,
            "modify_timestamp": now,
            "schema": schema,
            "events": empty_list.clone(),
            "event_queues": empty_list.clone(),
            "message_queues": empty_list
        };

        let result = block_on(manages_collection.insert_one(new_doc.clone(), None));
        match result {
            Ok(r) => {
                return Ok(new_doc);
            }
            Err(_e) => {
                return Err(operation_failed(
                    "new_manage_entity_to_database",
                    "插入管理实体失败",
                ))
            }
        }
    } else {
        return Err(operation_failed(
            "new_manage_entity_to_database",
            "管理实体已经存在",
        ));
    };
}

/// 读取所有管理
pub async fn get_manages_documents() -> Option<Vec<Document>> {
    //1. 取得所有管理，管理数量很少，总数一般不过百
    let manages_collection = database::get_manages_collection().await;
    let cusor = manages_collection
        .find(None, None)
        .await
        .expect("获取管理数据错误");

    //2. 构造
    let manages: Vec<MongodbResult<Document>> = cusor.collect().await;

    let result: Vec<Document> = manages.iter().map(|d| d.clone().unwrap()).collect();

    Some(result)
}

/// 管理集合是否存在
pub async fn manage_collection_exists(manage_name: &String) -> bool {
    let cashmere_db = database::get_cashmere_database().await;

    // 取得所有管理
    let exist_collections = cashmere_db
        .list_collection_names(None)
        .await
        .expect("集合取得错误");

    if exist_collections.contains(&manage_name) {
        return true;
    } else {
        return false;
    }
}

/// 管理条目是否存在
pub async fn manage_entity_exists(manage_id: &String) -> bool {
    let result = entity::exists_by_id(manage_id, &MANAGES_MANAGE_ID.to_string()).await;

    result
}

/// 取得管理实体
pub async fn get_manage_entity(manage_name: &String) -> Option<Document> {
    if !manage_collection_exists(&manage_name).await {
        return None;
    }

    let manage_entity = match entity::get_entity_by_name(&"manages".to_string(), manage_name).await
    {
        Ok(r) => r,
        Err(_e) => return None,
    };

    Some(manage_entity)
}

/*
   关联事件队列
   操作在管理条目上
*/
pub async fn associate_event_queue(
    manage_name: String,
    queue_name: String,
) -> Result<bool, String> {
    let manage_collection = database::get_manages_collection().await;

    let mut manage = match get_manage_entity(&manage_name).await {
        Some(d) => d,
        None => return Err("associate failed".to_string()),
    };

    let queue = match get_manage_entity(&queue_name).await {
        Some(d) => d,
        None => return Err("associate failed".to_string()),
    };

    let queue_id = queue.get("_id").unwrap();
    let manage_event_queues = manage.get_array_mut("event_queues").unwrap();
    manage_event_queues.push(queue_id.clone());

    let result = manage_collection
        .update_one(
            doc! {
                "name": manage_name,
            },
            doc! {
               "$set": { "event_queues" :  bson::Bson::Array(manage_event_queues.clone()) }
            },
            None,
        )
        .await;

    match result {
        Ok(r) => Ok(true),
        Err(e) => Err("associate event queue failed".to_string()),
    }
}
