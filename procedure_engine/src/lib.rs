/*
Author: 闫刚 (yes7rose@sina.com)
lib.rs (c) 2020
Desc: 过程
Created:  2020-11-11T08:25:27.045Z
Modified: !date!
*/

mod phases;
mod work_node;
mod task;

use manage_define::manage_ids::PROCEDURES_MANAGE_ID;

use linked_hash_map::LinkedHashMap;
use mongodb::{bson, bson::doc, bson::Document};
use cash_result::*;
use serde::{Deserialize, Serialize};

use phases::Phase;


/// 任务图
#[derive(Debug, Deserialize, Serialize)]
struct WorkGraph(Vec<WorkNode>);
