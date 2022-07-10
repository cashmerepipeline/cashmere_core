use serde::{Deserialize, Serialize};

/// 映像结果
#[derive(Serialize, Deserialize)]
pub enum ViewRuleResult {
    // 不可读
    InVisible,
    // 全部可读
    Read,
    // 只组的 可读
    GroupRead,
    // 只主的 可读
    OwnerRead,
    // 全部可写
    Write,
    // 只组的可写
    GroupWrite,
    // 只主的 可写
    OwnerWrite,
}
