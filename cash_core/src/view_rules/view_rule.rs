use serde::{Serialize, Deserialize};

use super::{ReadRule, WriteRule, FilterRule};

/// 映像
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ViewRule {
    pub read_rule: ReadRule,
    pub write_rule: WriteRule,
    pub read_filters: Vec<FilterRule>,
    pub write_filters: Vec<FilterRule>,
}