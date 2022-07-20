use serde::{Deserialize, Serialize};

/// 过滤规则
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FilterRule {
    // 无限制
    NoLimit,

    //只有组的
    OnlyGroup,

    // 只有主的
    OnlyOwner,

    // 系统使用
    OnlySystem,
}
