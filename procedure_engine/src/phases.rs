/*
Author: 闫刚 (yes7rose@sina.com)
phases.rs (c) 2020
Desc: 过程阶段
Created:  2020-11-11T09:33:16.498Z
Modified: !date!
*/

/// 过程阶段，里程碑
// enum ProcedurePhase {
//     // 概念
//     Concepting,
//     // 概念验证
//     Proving,
//     // 设计
//     Designing,
//     // 测试
//     Testing,
//     // 实现
//     Implementation,
//     // 提高
//     Improving
// }

use serde::{Deserialize, Serialize};

/// 过程阶段
#[derive(Debug, Deserialize, Serialize)]
pub struct Phase{
    pub procedure_id: String,
    pub index: u8, 
    pub phase: String,
}
