use serde::{Deserialize, Serialize};

/// 读规则
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ReadRule {
    // 不可读
    InVisible,
    // 全部可读
    Read,
    // 只组的 可读
    GroupRead,
    // 只主的 可读
    OwnerRead,
}
