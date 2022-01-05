/*
Project: grpc_server
Creator: 闫刚
Create time: 2020-09-21 17:29
Modify time: 2020-09-21 17:29
Introduction:
*/
#![allow(unused_imports)]
#![allow(dead_code)]

mod utils;

use chrono::Utc;
use linked_hash_map::LinkedHashMap;
use mongodb::{bson, bson::doc, bson::Bson, bson::Document, Collection};
use serde::Deserialize;
use std::collections::BTreeMap;
// use tokio::stream::StreamExt;
use futures::stream::StreamExt;

use cash_result::*;
use manage_define::general_field_ids::*;

use database::get_cashmere_database;
use mongodb::options::{FindOneAndUpdateOptions, UpdateOptions};

/// 取得新连续id
pub async fn get_new_entity_id(manage_id: &String, account_id: &String) -> Option<i64> {
    let ids_collection = database::get_ids_collection().await;
    let result = ids_collection
        .find_one_and_update(
            doc! {
                "_id": manage_id.clone()
            },
            doc! {
                "$inc": {"id_count":1},
                "$set": {
                    MODIFIER_FIELD_ID.to_string(): account_id.clone(),
                    MODIFY_TIMESTAMP_FIELD_ID.to_string(): Utc::now().timestamp()
                }
            },
            Some(FindOneAndUpdateOptions::builder().upsert(true).build()),
        )
        .await;

    match result {
        Ok(r) => Some(r.unwrap().get_i32("id_count").unwrap() as i64),
        Err(_e) => None,
    }
}

pub async fn entity_exists(manage_id: &String, query_doc: Document) -> bool {
    // 检查
    let collection = match database::get_collection_by_id(manage_id).await {
        Some(c) => c,
        None => return false,
    };
    let result = collection.find_one(query_doc, None).await;

    match result {
        Ok(Some(_r)) => true,
        Ok(None) => false,
        Err(_e) => false,
    }
}

/// 根据名字判断条目是否存在
pub async fn exists_by_name(entity_name: &String, manage_id: &String) -> bool {
    let query_doc = doc! {
        "name": entity_name.clone()
    };

    entity_exists(manage_id, query_doc).await
}

pub async fn exists_by_id(manage_id: &String, entity_id: &String) -> bool {
    let query_doc = doc! {
        "_id": entity_id.clone()
    };

    entity_exists(manage_id, query_doc).await
}

/// 插入实体
pub async fn insert_entity(
    manage_id: &String,
    entity_doc: &mut Document,
    account_id: &String,
    group_id: &String,
) -> Result<OperationResult, OperationResult> {
    // 检查
    let collection = match database::get_collection_by_id(manage_id).await {
        Some(c) => c,
        None => return Err(collection_not_exists("insert_entity")),
    };

    if entity_doc.contains_key(&ID_FIELD_ID.to_string()) {
        let v = entity_doc
            .get_str(&ID_FIELD_ID.to_string())
            .unwrap()
            .to_string();
        entity_doc.insert("_id", v);
    }

    // 创建标记
    entity_doc.insert(CREATOR_FIELD_ID.to_string(), account_id.clone());
    entity_doc.insert(MODIFIER_FIELD_ID.to_string(), account_id.clone());
    entity_doc.insert(OWNER_FIELD_ID.to_string(), account_id.clone());
    entity_doc.insert(GROUPS_FIELD_ID.to_string(), vec![group_id.clone()]);
    entity_doc.insert(
        MODIFY_TIMESTAMP_FIELD_ID.to_string(),
        Utc::now().timestamp(),
    );
    entity_doc.insert(
        CREATE_TIMESTAMP_FIELD_ID.to_string(),
        Utc::now().timestamp(),
    );

    // 插入
    let result = collection.insert_one(entity_doc.clone(), None).await;

    // 结果
    match result {
        Ok(r) => match r.inserted_id.as_object_id() {
            Some(id) => Ok(operation_succeed(id.to_string())),
            None => Ok(operation_succeed("succeed")),
        },
        Err(_e) => Err(operation_failed(
            "insert_entity",
            format!("插入实体失败 {}", entity_doc),
        )),
    }
}

/// 变更条目所属人
pub async fn change_entity_owner(
    manage_id: &String,
    entity_id: &String,
    new_owner: &String,
    account_id: &String,
) -> Result<OperationResult, OperationResult> {
    let _new_value = Bson::from(new_owner);

    let query_doc = doc! {"_id":entity_id.clone()};
    let modify_doc = doc! {OWNER_FIELD_ID.to_string():new_owner.clone()};

    let result = update_entity_field(manage_id, query_doc, modify_doc, account_id).await;

    result
}

/// 变更条目所属组
pub async fn update_entity_groups(
    manage_id: &String,
    entity_id: &String,
    new_groups: &Vec<String>,
    account_id: &String,
) -> Result<OperationResult, OperationResult> {
    // 集合是否存在， 不自动创建集合
    let _collection = match database::get_collection_by_id(manage_id).await {
        Some(c) => c,
        None => return Err(collection_not_exists("update_entity_groups")),
    };

    let query_doc = doc! {
        "_id": entity_id
    };

    let modify_doc = doc! {
        GROUPS_FIELD_ID.to_string(): { "$each":new_groups}
    };

    push_entity_array_field(manage_id, query_doc, modify_doc, account_id).await
}

/// 更新实体单个属性
pub async fn update_entity_field(
    manage_id: &String,
    query_doc: Document,
    modify_doc: Document,
    account_id: &String,
) -> Result<OperationResult, OperationResult> {
    // 集合是否存在， 不自动创建集合
    let collection = match database::get_collection_by_id(manage_id).await {
        Some(c) => c,
        None => return Err(collection_not_exists("update_entity_field")),
    };

    // 更新
    let result = collection
        .update_one(
            query_doc.clone(),
            doc! {
            "$set": modify_doc,
                "$set": {
                    MODIFIER_FIELD_ID.to_string(): account_id.clone(),
                    MODIFY_TIMESTAMP_FIELD_ID.to_string(): Utc::now().timestamp()
                }
            },
            None,
        )
        .await;

    // 结果
    match result {
        Ok(r) => match r.modified_count == 1 {
            true => Ok(operation_succeed("succeed")),
            false => Err(operation_failed(
                "updata_entity",
                format!("更新了多个实体{}", query_doc),
            )),
        },
        Err(_e) => Err(operation_failed(
            "update_entity",
            format!("更新操作失败{}", query_doc),
        )),
    }
}

// --------------------------
// 数组属性操作
// --------------------------

///  添加单个新元素
pub async fn push_entity_array_field(
    manage_id: &String,
    query_doc: Document,
    modify_doc: Document,
    account_id: &String,
) -> Result<OperationResult, OperationResult> {
    // 集合是否存在， 不自动创建集合
    let collection = match database::get_collection_by_id(manage_id).await {
        Some(c) => c,
        None => return Err(collection_not_exists("push_entity_array_field")),
    };

    // 更新
    let result = collection
        .update_one(
            query_doc.clone(),
            doc! {
                // 添加
                "$addToSet": modify_doc,
                "$set": {
                    MODIFIER_FIELD_ID.to_string(): account_id.clone(),
                    MODIFY_TIMESTAMP_FIELD_ID.to_string(): Utc::now().timestamp()
                }
            },
            None,
        )
        .await;

    // 结果
    match result {
        Ok(r) => match r.modified_count == 1 {
            true => Ok(operation_succeed("succeed")),
            false => Err(operation_failed(
                "push_entity_array_field",
                format!("更新了多个实体{}", query_doc),
            )),
        },
        Err(_e) => Err(operation_failed(
            "push_entity_array_field",
            format!("添加操作失败{}", query_doc),
        )),
    }
}

///  列表属性 移除元素
pub async fn pull_entity_array_field(
    manage_id: &String,
    query_doc: Document,
    modify_doc: Document,
    account_id: &String,
) -> Result<OperationResult, OperationResult> {
    // 集合是否存在， 不自动创建集合
    let collection = match database::get_collection_by_id(manage_id).await {
        Some(c) => c,
        None => return Err(collection_not_exists("push_entity_array_field")),
    };

    // 更新
    let result = collection
        .update_one(
            query_doc.clone(),
            doc! {
                "$pull": modify_doc,
                "$set": {
                    MODIFIER_FIELD_ID.to_string(): account_id.clone(),
                    MODIFY_TIMESTAMP_FIELD_ID.to_string(): Utc::now().timestamp()
                }
            },
            None,
        )
        .await;

    // 结果
    match result {
        Ok(r) => match r.modified_count == 1 {
            true => Ok(operation_succeed("succeed")),
            false => Err(operation_failed(
                "pop_entity_array_field",
                format!("更新了多个实体{}", query_doc),
            )),
        },
        Err(_e) => Err(operation_failed(
            "pop_entity_array_field",
            format!("删除操作失败{}", query_doc),
        )),
    }
}

/// 更新元素
pub async fn update_entity_array_field(
    manage_id: &String,
    query_doc: Document,
    modify_doc: Document,
    account_id: &String,
) -> Result<OperationResult, OperationResult> {
    // 集合是否存在， 不自动创建集合
    let collection = match database::get_collection_by_id(manage_id).await {
        Some(c) => c,
        None => return Err(collection_not_exists("upate_entity_array_field")),
    };

    // 更新
    let result = collection
        .update_one(
            query_doc.clone(),
            doc! {
                "$set": modify_doc,
                "$set": {
                    MODIFIER_FIELD_ID.to_string(): account_id.clone(),
                    MODIFY_TIMESTAMP_FIELD_ID.to_string(): Utc::now().timestamp()
                }
            },
            None,
        )
        .await;

    // 结果
    match result {
        Ok(r) => match r.modified_count == 1 {
            true => Ok(operation_succeed("succeed")),
            false => Err(operation_failed(
                "update_entity_array_field",
                format!("更新了多个实体{}", query_doc),
            )),
        },
        Err(_e) => Err(operation_failed(
            "update_entity_array_field",
            format!("更新操作失败{}", query_doc),
        )),
    }
}

/// 更新map元素
pub async fn update_entity_array_map_field(
    manage_id: &String,
    query_doc: Document,
    modify_doc: Document,
    account_id: &String,
) -> Result<OperationResult, OperationResult> {
    // 集合是否存在， 不自动创建集合
    let collection = match database::get_collection_by_id(manage_id).await {
        Some(c) => c,
        None => return Err(collection_not_exists("upate_entity_array_field")),
    };

    // 更新
    let result = collection
        .update_one(
            query_doc.clone(),
            doc! {
                "$set": modify_doc,
                "$set": {
                    MODIFIER_FIELD_ID.to_string(): account_id.clone(),
                    MODIFY_TIMESTAMP_FIELD_ID.to_string(): Utc::now().timestamp()
                }
            },
            None,
        )
        .await;

    // 结果
    match result {
        Ok(r) => match r.modified_count == 1 {
            true => Ok(operation_succeed("succeed")),
            false => Err(operation_failed(
                "update_entity_array_field",
                format!("更新了多个实体{}", query_doc),
            )),
        },
        Err(_e) => Err(operation_failed(
            "update_entity_array_field",
            format!("更新操作失败{}", query_doc),
        )),
    }
}

/// 更新实体多个个属性
pub async fn update_entity_fields(
    entity_id: &String,
    collection: &String,
    new_value: Document,
    account_id: &String,
) -> Result<OperationResult, OperationResult> {
    // 集合是否存在， 不自动创建集合
    let collection = match database::get_collection_by_id(collection).await {
        Some(c) => c,
        None => return Err(collection_not_exists("update_entity")),
    };

    // 更新
    let result = collection
        .update_one(
            doc! {
                "_id": entity_id
            },
            doc! {
                "$set": new_value,
                MODIFIER_FIELD_ID.to_string(): account_id.clone(),
                MODIFY_TIMESTAMP_FIELD_ID.to_string(): Utc::now().timestamp()
            },
            None,
        )
        .await;

    // 结果
    match result {
        Ok(r) => match r.modified_count == 1 {
            true => Ok(operation_succeed("succeed")),
            false => Err(operation_failed(
                "updata_entity",
                format!("更新了多个实体{}", entity_id),
            )),
        },
        Err(_e) => Err(operation_failed(
            "update_entity",
            format!("更新操作失败{}", entity_id),
        )),
    }
}

// --------------------------
// 映射属性操作
// --------------------------
pub async fn insert_entity_map_field(
    manage_id: &String,
    query_doc: Document,
    modify_doc: Document,
    account_id: &String,
) -> Result<OperationResult, OperationResult> {
    // 集合是否存在， 不自动创建集合
    let collection = match database::get_collection_by_id(manage_id).await {
        Some(c) => c,
        None => return Err(collection_not_exists("update_entity")),
    };

    // 更新
    let result = collection
        .update_one(
            query_doc.clone(),
            doc! {
                "$set":modify_doc,
                "$set": {
                MODIFIER_FIELD_ID.to_string(): account_id.clone(),
                MODIFY_TIMESTAMP_FIELD_ID.to_string(): Utc::now().timestamp()
                }
            },
            UpdateOptions::builder().upsert(true).build(),
        )
        .await;

    // 结果
    match result {
        Ok(r) => match r.modified_count == 1 {
            true => Ok(operation_succeed("ok")),
            false => Err(operation_failed(
                "insert_entity_map_field",
                format!("更新了多个实体{}", query_doc),
            )),
        },
        Err(_e) => Err(operation_failed(
            "insert_entity_map_field",
            format!("更新操作失败{}", query_doc),
        )),
    }
}

/// 更新map字段
pub async fn update_entity_map_field(
    manage_id: &String,
    query_doc: Document,
    modify_doc: Document,
    account_id: &String,
) -> Result<OperationResult, OperationResult> {
    insert_entity_map_field(manage_id, query_doc, modify_doc, account_id).await
}

/// 取得 实体
pub async fn get_entity_by_id(
    collection_name: &String,
    id: &String,
) -> Result<Document, OperationResult> {
    if !database::collection_exists(collection_name).await {
        return Err(collection_not_exists("get_entity_by_id"));
    }

    let collection = match database::get_collection_by_id(collection_name).await {
        Some(c) => c,
        None => return Err(collection_not_exists("get_entity_by_id")),
    };

    let result = collection
        .find_one(
            doc! {
                "_id": id
            },
            None,
        )
        .await;

    match result {
        Ok(r) => match r {
            Some(d) => Ok(d),
            None => Err(operation_failed(
                "get_entity_by_id",
                format!("无数据 {}", id),
            )),
        },
        Err(_e) => Err(operation_failed(
            "get_entity_by_id",
            format!("获取失败{}", id),
        )),
    }
}

/// 根据名字取得entity
pub async fn get_entity_by_name(
    collection_name: &String,
    name: &String,
) -> Result<Document, OperationResult> {
    let collection = match database::get_collection_by_id(collection_name).await {
        Some(c) => c,
        None => return Err(collection_not_exists("get_entity_by_id")),
    };

    let result = collection
        .find_one(
            doc! {
                "name": name.clone()
            },
            None,
        )
        .await;

    match result {
        Ok(r) => Ok(r.unwrap()),
        Err(_e) => Err(operation_failed(
            "get_entity_by_id",
            format!("获取失败{}", name),
        )),
    }
}

pub async fn get_entities(
    collection_name: &String,
    filter: &Option<Document>,
) -> Result<Vec<Document>, OperationResult> {
    let collection = match database::get_collection_by_id(collection_name).await {
        Some(c) => c,
        None => return Err(collection_not_exists("get_entities")),
    };

    let cursor = collection.find(filter.clone(), None).await;

    let mut result: Vec<Document> = Vec::new();
    match cursor {
        Ok(mut r) => {
            while let Some(d) = r.next().await {
                match d {
                    Ok(dc) => result.push(dc),
                    _ => continue,
                }
            }
            Ok(result)
        }
        Err(_e) => Err(operation_failed(
            "get_entities",
            format!("获取失败{}", filter.clone().unwrap_or_default()),
        )),
    }
}

/// 取得条件排序分页
pub async fn get_entities_by_page(
    collection_name: &String,
    page_index: u32,
    matches: &Option<Document>,
    conditions: &Document,
) -> Result<Vec<Document>, OperationResult> {
    let collection = match database::get_collection_by_id(collection_name).await {
        Some(c) => c,
        None => return Err(collection_not_exists("get_entities_by_page")),
    };

    let mut pipeline: Vec<Document> = vec![];
    if matches.is_some() {
        pipeline.push(doc! {"$match": matches});
    }
    pipeline.push(doc! {"$sort": conditions.clone()});
    pipeline.push(doc! {"$limit": 20});
    let cursor = collection.aggregate(pipeline, None).await;

    let mut result: Vec<Document> = Vec::new();
    match cursor {
        Ok(mut r) => {
            while let Some(d) = r.next().await {
                match d {
                    Ok(dc) => result.push(dc),
                    _ => continue,
                }
            }
            Ok(result)
        }
        Err(_e) => Err(operation_failed(
            "get_entities_by_page",
            format!("获取分页失败{}-{}", page_index, conditions),
        )),
    }
}

/// 取得实体属性
pub fn get_entity_field(entity_doc: &Document, field_name: impl Into<String>) -> Option<Bson> {
    entity_doc.get(field_name.into().as_str()).cloned()
}

/// 取得实体属性为String
pub fn get_entity_field_as_string(
    entity_doc: &Document,
    field_name: impl Into<String>,
) -> Option<String> {
    let b = match get_entity_field(entity_doc, field_name) {
        Some(r) => r,
        None => return None,
    };

    b.as_str().map(|result| result.to_string())
}

/// 取得实体属性到对应类型
pub fn get_entity_field_as_type<'a, T: Deserialize<'a>, U: for<'de> serde::Deserialize<'de>>(
    entity_doc: &'a Document,
    field_name: impl Into<String>,
) -> Option<U> {
    let b = match get_entity_field(entity_doc, field_name) {
        Some(r) => r,
        None => return None,
    };

    Some(bson::from_bson::<U>(b).unwrap())
}

/// 根据服务器的语言设置取得实体名
pub fn get_entity_name(entity_doc: &Document) -> Option<String> {
    let lang_code = configs::get_lang_code();

    let name = match get_entity_field(entity_doc, &NAME_FIELD_ID.to_string()) {
        Some(b) => b,
        None => return None,
    };

    if let Some(n) = name.as_document() {
        if n.contains_key(lang_code.as_str()) {
            Some(n.get(lang_code.as_str()).unwrap().to_string())
        } else {
            // 如果没有，使用第一个名字

            let mut name_b: Bson = Bson::String("".to_string());
            // for x in n {
            //     name_b = x.1.clone();
            //     break;
            // }
            if let Some(first_name) = n.into_iter().next() {
                name_b = first_name.1.clone();
            }
            Some(name_b.as_str().unwrap().to_string())
        }
    } else {
        None
    }
}

/// 取得实体编号
pub fn get_entity_id(entity_doc: &Document) -> Option<String> {
    let id = match get_entity_field(entity_doc, "_id") {
        Some(b) => b,
        None => return None,
    };

    id.as_str().map(|r| r.to_string())
}

/// 取得实体数据 所属人
pub fn get_entity_owner(entity_doc: &Document) -> Option<String> {
    match entity_doc.get_str("owner") {
        Ok(r) => Some(r.to_string()),
        Err(_e) => None,
    }
}

/// 取得实体数据 所属组
pub fn get_entity_groups(entity_doc: &Document) -> Option<Vec<String>> {
    match entity_doc.get_array("groups") {
        Ok(r) => Some(r.iter().map(|x| x.as_str().unwrap().to_string()).collect()),
        Err(_e) => None,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
