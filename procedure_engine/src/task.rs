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


pub struct Task{
    // 工作节点id
    node: String,
    status: TaskStatus,
}