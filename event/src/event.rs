/*
Author: 闫刚 (yes7rose@sina.com)
event.rs (c) 2020
Desc: 事件
Created:  2020-11-08T03:25:10.174Z
Modified: !date!
*/

use bson::{self, doc, Bson, Document};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::stream::StreamExt;

use cash_core;
use cash_core::ids::EVENTS_MANAGE_ID;
use cash_core::results::*;
use entity;

use crate::events_map::get_events_map;

const TARGET_QUEUES: i32 = 1008;

/// 发送者
#[derive(Debug, Clone)]
pub struct Emitor {
    manage_id: i32,
    entity_id: String,
}

/// 事件上下文
#[derive(Debug, Clone)]
pub struct Context {
    trig_time: i64,
    data: Document,
}

/// 事件
#[derive(Debug, Clone)]
pub struct Event {
    pub event_id: String,
    pub emitor: Emitor,
    pub context: Option<Context>,
}

// 目标队列
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTargetQueue {
    manage: String,
    queue: String,
}

/// 取得新事件id
pub async fn get_event_new_id() -> i64 {
    let events_map_arc = get_events_map().await;
    let events_map_lock = events_map_arc.read();
    let len = events_map_lock.keys().len() as i64;

    //自动生成编号在5000以上
    len + 5000 + 1
}

/// 添加事件
// pub async fn new_event(
//     manage_id: &String,
//     event_name: &String,
//     account_id: &String,
//     group_id: &String,
//     id: Option<i32>,
// ) -> Result<OperationResult, OperationResult> {
//     let event_manage_id = &EVENTS_MANAGE_ID.to_string();
//     let mut entity_doc = doc! {
//         "_id": id.unwrap().to_string(),
//         "manage": manage_id.clone(),
//         "name": event_name.clone(),
//     };
//
//     let id = match id {
//         Some(r) => r.to_string(),
//         None => get_event_new_id().await.to_string(),
//     };
//
//     // 添加到管理数据库
//     match entity::insert_entity(event_manage_id, &mut entity_doc, account_id, group_id).await {
//         Ok(_r) => _r,
//         Err(e) => return Err(e),
//     };
//     // 添加到事件全局缓存Map
//     // 格式: {event_id：queues}
//     let mut events_map = get_events_map().await.write();
//     events_map.insert(id,)
//     let mut manage_event_que_map = if events_map.contains_key(manage_id) {
//         events_map.get_mut(manage_id).unwrap()
//     } else {
//         events_map.insert(
//             manage_id.clone(),
//             HashMap::<String, Vec<EventTargetQueue>>::new(),
//         );
//         events_map.get_mut(manage_id).unwrap()
//     };
//
//     // 存在返回信息，不存在添加
//     if manage_event_que_map.contains_key(event_name) {
//         return Err(target_already_exists("new_event"));
//     } else {
//         manage_event_que_map.insert(event_name.clone(), vec![]);
//         return Ok(operation_succeed(id));
//     }
// }

/// 添加事件目标队列
// pub async fn add_event_target_queue(
//     event_id: &String,
//     queue_id: &String,
// ) -> Result<OperationResult, OperationResult> {
//     let target_q = EventTargetQueue {
//         manage: target_manage_id.clone(),
//         queue: queue_id.clone(),
//     };
//     // 更新到管理数据库
//     match push_event_target_queue(event_id, event_name, &target_q).await {
//         Err(e) => return Err(e),
//         _ => None::<OperationResult>,
//     };
//
//     // 添加到缓存
//     let mut events_map = get_events_map().await.write();
//     let mut queues = events_map
//         .get_mut(event_id)
//         .unwrap()
//         .get_mut(event_name)
//         .unwrap();
//
//     queues.push(target_q);
//
//     Ok(operation_succeed("succeed"))
// }

/// 更新事件目标队列
pub async fn push_event_target_queue(
    event_id: &String,
    event_name: &String,
    target_que: &EventTargetQueue,
    account_id: &String
) -> Result<OperationResult, OperationResult> {
    let new_values: Bson = bson::to_bson(target_que).unwrap();
    entity::push_entity_array_field(
        &EVENTS_MANAGE_ID.to_string(),
        &event_id,
        &TARGET_QUEUES.to_string(),
        &new_values,
        account_id
    )
    .await
}

// 取得事件实体
pub async fn get_event_entity(
    manage: &String,
    event: &String,
) -> Result<Document, OperationResult> {
    let collection = match database::get_collection_by_id(&"events".to_string()).await {
        Some(r) => r,
        None => return Err(operation_failed("get_event_entity", "事件集合不存在")),
    };
    let result = match collection
        .find_one(
            doc! {
                "name": event.clone(),
                "manage": manage.clone()
            },
            None,
        )
        .await
    {
        Ok(r) => r,
        Err(_e) => return Err(operation_failed("get_event_entity", "查询事件错误")),
    };
    match result {
        Some(r) => Ok(r),
        None => Err(operation_failed("get_event_entity", "事件不存在")),
    }
}

// /// 发送事件
// pub async fn send_event_to_queue(
//     event: &Event,
//     target_queue_id: &i32,
// ) -> Result<OperationResult, OperationResult> {
//     match get_event_queue_sender(target_queue_id).await {
//         Some(mut r) => {
//             r.send(event.clone());
//             Ok(operation_succeed("succeed"))
//         }
//         None => Err(operation_failed("send_event_to_queue", "发送事件失败")),
//     }
// }
