/*
Author: 闫刚 (yes7rose@sina.com)
work_grpah.rs (c) 2020
Desc: 工作图
Created:  2020-11-15T09:30:04.425Z
Modified: !date!
*/

use manage_define::manage_ids::WORK_NODES_MANAGE_ID;
use database;
use entity;
use cash_result::*;
use mongodb::bson::{doc};

use super::task;

/// 取得新编号
pub async fn get_new_work_node_id() -> Result<u64, OperationResult> {
    let collect =
        match database::get_collection_by_id(&WORK_NODES_MANAGE_ID.to_string()).await {
            Some(r) => r,
            None => return Err(operation_failed("get_new_work_graph_id", "取得工作图集合错误"))
        };

    let count = match collect.estimated_document_count(None).await {
        Ok(r) => r,
        Err(_e) => return Err(operation_failed("get_new_work_graph_id", "取得工作图数量错误"))
    };

    Ok(count + 1 + 10000)
}

/// 新建工作节点
pub async fn new_work_node(
    work_graph_id: &String,
    node_id: &String,
    work_name: &String,
    account_id: &String,
    group_id: &String,
) -> Result<OperationResult, OperationResult> {
    let mut new_node_doc =
        doc! {
            "_id": node_id.clone(),
            "name": work_name.clone(),
            "work_graph": work_graph_id.clone(),
        };

    entity::insert_entity(
        &WORK_NODES_MANAGE_ID.to_string(),
        &mut new_node_doc,
        account_id,
        group_id,
    ).await
}

/// 添加新任务
pub async fn add_new_task(
    work_node_id: &String,
    task_id: &String,
    task_name: &String,
    account_id: &String,
    group_id: &String,
) -> Result<OperationResult, OperationResult> {
    let new_id = 
        match task::get_new_task_id().await {
            Ok(r) => r,
            Err(e) => return Err(e)
        };

    task::new_task(
        work_node_id,
        task_id,
        task_name,
        account_id, 
        group_id).await
}