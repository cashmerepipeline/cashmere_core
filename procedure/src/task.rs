/*
Author: 闫刚 (yes7rose@sina.com)
lib.rs (c) 2020
Desc: 任务
Created:  2020-11-11T08:25:27.045Z
Modified: !date!
*/

use  manage_define::manage_ids::TASKS_MANAGE_ID;


use cash_result::*;
use mongodb::bson::{doc};

pub enum TaskStatus {
    WaitStarting,
    Working,
    Done
}

pub struct Task{
    // 工作节点id
    node: String,
    status: TaskStatus,
}



/// 取得新编号
pub async fn get_new_task_id() -> Result<u64, OperationResult> {
    let collect =
        match database::get_collection_by_id(&TASKS_MANAGE_ID.to_string()).await {
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
pub async fn new_task(
    work_node_id: &String,
    task_id: &String,
    work_name: &String,
    account_id: &String,
    group_id: &String,
) -> Result<OperationResult, OperationResult> {
    let mut new_task_doc =
        doc! {
            "_id": task_id.clone(),
            "name": work_name.clone(),
            "work_node": work_node_id.clone(),
        };

    entity::insert_entity(
        &TASKS_MANAGE_ID.to_string(),
        &mut new_task_doc,
        account_id,
        group_id
    ).await
}