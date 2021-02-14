/*
Author: 闫刚 (yes7rose@sina.com)
lib.rs (c) 2020
Desc: 过程
Created:  2020-11-11T08:25:27.045Z
Modified: !date!
*/

mod phases;
mod work_graph;
mod work_node;
mod task;

use manage_define::manage_ids::PROCEDURES_MANAGE_ID;
use database;
use entity;

use linked_hash_map::LinkedHashMap;
use mongodb::{bson, bson::doc, bson::Document};
use cash_result::*;
use serde::{Deserialize, Serialize};

use phases::Phase;

/// 任务节点
#[derive(Debug, Deserialize, Serialize)]
pub struct WorkNode {
    procedure_id: String,
    // pre task ids
    pub pre_nodes: Vec<String>,
    pub this_tasks: Vec<String>,
    // post task ids
    pub post_nodes: Vec<String>,
}

/// 任务图
#[derive(Debug, Deserialize, Serialize)]
struct WorkGraph(Vec<WorkNode>);

/// 过程
struct Procedure {
    // id
    work_id: String,
    phase: Option<Phase>,
    // 不同阶段对应不同工作图
    work_graphs: Option<LinkedHashMap<String, WorkGraph>>,
}

/// 新建过程
pub async fn new_procedure(
    work_id: &String,
    phase_set_id: &String,
    account_id: &String,
    group_id: &String,
) -> Result<OperationResult, OperationResult> {
    let work_graphs: Option<LinkedHashMap<String, WorkGraph>> = None;
    let mut new_doc = doc! {
        "work": work_id,
        "phase_set": phase_set_id,
        "current_phase": 0i32,
        "work_graphs": bson::to_bson(&work_graphs).unwrap()
    };

    let result = entity::insert_entity(
        &PROCEDURES_MANAGE_ID.to_string(),
        &mut new_doc,
        account_id,
        group_id,
    )
        .await;


    result
}

/// 取得过程
pub async fn get_procedure(
    procudure_id: &String
) -> Result<Document, OperationResult> {
    entity::get_entity_by_id(&PROCEDURES_MANAGE_ID.to_string(), procudure_id).await
}

// /// 设置过程阶段集
// pub async fn set_procedure_phase_set(
//     procudure_id: &String,
//     phase_set_id: &String,
// ) -> Result<OperationResult, OperationResult> {
//     let new_value = bson::to_bson(phase_set_id).unwrap();
//
//     entity::update_entity_field(
//         &PROCEDURES_MANAGE_ID.to_string(),
//         procudure_id,
//         "phase_set", new_value).await
// }
//
// /// 修改过程阶段
// pub async fn change_procedure_phase(
//     procudure_id: &String,
//     new_phase_index: &i32,
//     account_id:&String
// ) -> Result<OperationResult, OperationResult> {
//     let new_value = bson::to_bson(new_phase_index).unwrap();
//
//     entity::update_entity_field(
//         &PROCEDURES_MANAGE_ID.to_string(),
//         procudure_id,
//         "current_phase", new_value,
//         account_id
//     ).await
// }

// /// 添加工作图
// pub async fn add_procedure_work_graph(
//     procudure_id: &String,
//     graph_name: &String,
//     account_id: &String,
//     group_id: &String,
// ) -> Result<OperationResult, OperationResult> {
//     let new_graph_id =
//         match work_graph::get_new_work_graph_id().await{
//             Ok(r) => r,
//             Err(e) => return Err(e)
//         };

//     work_graph::new_work_graph(
//         procudure_id,
//         &new_graph_id.to_string(),
//         graph_name,
//         account_id,
//         group_id).await
// }


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
