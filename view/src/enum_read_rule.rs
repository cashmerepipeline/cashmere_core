use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

/// 读规则
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ReadRule {
    // 不可读
    InVisible,
    // 全部可读
    Read,
    // 只组的可读
    GroupRead,
    // 只主的可读
    OwnerRead,
    // 未知
    Unknown,
}

impl Display for ReadRule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadRule::InVisible => write!(f, "InVisible"),
            ReadRule::Read => write!(f, "Read"),
            ReadRule::GroupRead => write!(f, "GroupRead"),
            ReadRule::OwnerRead => write!(f, "OwnerRead"),
            ReadRule::Unknown => write!(f, "Unknown"),
        }
    }
}

impl From<String> for ReadRule {
    fn from(s: String) -> Self {
        let invisible = String::from("InVisible");
        let read = String::from("Read");
        let group_read = String::from("GroupRead");
        let owner_read = String::from("OwnerRead");
        match s {
            s if s == invisible => ReadRule::InVisible,
            s if s == read => ReadRule::Read,
            s if s == group_read => ReadRule::GroupRead,
            s if s == owner_read => ReadRule::OwnerRead,
            _ => ReadRule::Unknown
        }
    }
}