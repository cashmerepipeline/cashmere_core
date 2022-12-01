use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum WriteRule {
    // 只读
    InVisible,
    // 全部可写
    Write,
    // 只组的可写
    GroupWrite,
    // 只主的 可写
    OwnerWrite,
}