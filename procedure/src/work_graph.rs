/*
Author: 闫刚 (yes7rose@sina.com)
work_grpah.rs (c) 2020
Desc: 工作图
Created:  2020-11-15T09:30:04.425Z
Modified: !date!
*/

// use database;
// use entity;
// use cash_core::results::*;
// use mongodb::bson::{doc, Document};
// use super::work_node;

// /// 取得新编号
// pub async fn get_new_work_graph_id() -> Result<i64, OperationResult> {
//     let collect =
//         match database::get_collection_by_id(&WORK_GRAPHS_MANAGE_ID.to_string()).await {
//             Some(r) => r,
//             None => return Err(operation_failed("get_new_work_graph_id", "取得工作图集合错误"))
//         };

//     let count = match collect.estimated_document_count(None).await {
//         Ok(r) => r,
//         Err(_e) => return Err(operation_failed("get_new_work_graph_id", "取得工作图数量错误"))
//     };

//     Ok(count + 1 + 10000)
// }

// /// 新建工作图
// pub async fn new_work_graph(
//     procedure_id: &String,
//     new_id: &String,
//     name: &String,
//     account_id: &String,
//     group_id: &String,
// ) -> Result<OperationResult, OperationResult> {
//     let mut new_work_graph_doc =
//         doc! {
//             "_id": new_id.clone(),
//             "procedure": procedure_id.clone(),
//         };

//     entity::insert_entity(
//         &WORK_GRAPHS_MANAGE_ID.to_string(),
//         &mut new_work_graph_doc,
//         account_id,
//         group_id,
//     ).await
// }

// /// 添加新工作节点到工作图
// pub async fn add_new_work_node(
//     work_graph_id: &String,
//     work_node_name: &String,
//     account_id: &String,
//     group_id: &String
// ) -> Result<OperationResult, OperationResult> {
//     let node_id =
//        match work_node::get_new_work_node_id().await {
//         Ok(r) => r,
//         Err(e) => return Err(e)
//     };

//     work_node::new_work_node(
//         work_graph_id,
//         &node_id.to_string(),
//         work_node_name,
//         account_id,
//         group_id
//     ).await
// }