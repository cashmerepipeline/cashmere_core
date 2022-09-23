use std::fmt::{Display, Formatter, write};
use serde::{Deserialize, Serialize};

/// 写规则
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum WriteRule {
    // 只读
    InVisible,
    // 全部可写
    Write,
    // 只组的可写
    GroupWrite,
    // 只主的可写
    OwnerWrite,
    // 未知
    Unknown,
}

impl Display for WriteRule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WriteRule::InVisible => write!(f, "InVisible"),
            WriteRule::Write => write!(f, "Write"),
            WriteRule::GroupWrite => write!(f, "GroupWrite"),
            WriteRule::OwnerWrite => write!(f, "OwnerWrite"),
            WriteRule::Unknown => write!(f, "Unknown"),
        }
    }
}

impl From<String> for WriteRule {
    fn from(s: String) -> Self {
        match s {
            s if s == String::from("InVisible") => WriteRule::InVisible,
            s if s == String::from("Write") => WriteRule::Write,
            s if s == String::from("GroupWrite") => WriteRule::GroupWrite,
            s if s == String::from("OwnerWrite") => WriteRule::OwnerWrite,
            _ => WriteRule::Unknown
        }
    }
}
