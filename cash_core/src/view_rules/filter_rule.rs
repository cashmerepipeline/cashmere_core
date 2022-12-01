use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FilterRule {
    NoLimit,
    // 主 所有
    OwnerALl,
    // 组 所有
    GroupALl,
    //只有组的
    OnlyGroup,
    // 只有主的
    OnlyOwner,
}