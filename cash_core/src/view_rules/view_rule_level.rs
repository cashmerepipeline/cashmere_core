use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

// 映像规则层级
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ViewRuleLevel{
    Manage,
    Collection,
    Schema
}

// 层级
impl Display for ViewRuleLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ViewRuleLevel::Manage => write!(f, "Manage"),
            ViewRuleLevel::Collection => write!(f, "Collection"),
            ViewRuleLevel::Schema => write!(f, "Schema"),
        }
    }
}