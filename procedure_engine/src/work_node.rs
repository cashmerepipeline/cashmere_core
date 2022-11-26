/*
Author: 闫刚 (yes7rose@sina.com)
work_grpah.rs (c) 2020
Desc: 工作图
Created:  2020-11-15T09:30:04.425Z
Modified: !date!
*/

use manage_define::manage_ids::WORK_NODES_MANAGE_ID;


use cash_result::*;
use mongodb::bson::{doc};

use super::task;

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
