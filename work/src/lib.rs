/*
Author: 闫刚 (yes7rose@sina.com)
lib.rs (c) 2020
Desc: 工作
Created:  2020-11-12T03:01:22.508Z
Modified: !date!
*/

mod graph;
mod node;
mod task;
mod procedure;
mod phases;

use cash_core::ids::WORKS_MANAGE_ID;
use database;
use entity;
use bson::{doc, Document};
use cash_core::results::*;

use serde::{Deserialize, Serialize};

/// 过程阶段
#[derive(Debug, Deserialize, Serialize)]
pub struct WorkPhase{
    pub index: i32,
    pub name: String,
}

/// 新建工作，直接添加到数据库，无缓存
// pub async fn new_work(
//     manage_id: &String,
//     account_id: &String,
//     group_id: &String,
// ) -> Result<OperationResult, OperationResult> {
//     let work_collection = database::get_collection_by_id(&WORKS_MANAGE_ID.to_string())
//         .await
//         .unwrap();
//
//     let mut new_entity_doc = doc! {
//         "manage": manage_id,
//         "procedure": ""
//     };
//
//     entity::insert_entity(
//         &WORKS_MANAGE_ID.to_string(),
//         &mut new_entity_doc,
//         account_id,
//         group_id,
//     )
//     .await
// }

/// 添加过程
// pub async fn add_precedure(
//     work_id: &String,
//     phase_set_id: &String,
//     account_id: &String,
//     group_id: &String,
// ) -> Result<OperationResult, OperationResult> {
//     procedure::new_procedure(work_id, phase_set_id, account_id, group_id).await
// }

// /// 取得工作过程, 返回过程实体
// pub async fn get_work_procedure(work_id: &String) -> Result<Document, OperationResult> {
//     let work_doc = match entity::get_entity_by_id(&WORKS_MANAGE_ID.to_string(), work_id).await {
//         Ok(r) => r,
//         Err(e) => return Err(e),
//     };
//
//     let pro_id = match entity::get_entity_field(&work_doc, "procedure") {
//         Some(r) => r.as_str().unwrap().to_string(),
//         None => return Err(field_not_exists("get_work_procedure", "procedure")),
//     };
//
//     procedure::get_procedure(&pro_id).await
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
