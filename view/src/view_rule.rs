use serde::{Deserialize, Serialize};

use crate::enum_filter_rule::FilterRule;
use crate::enum_read_rule::ReadRule;
use crate::enum_write_rule::WriteRule;

/// 映像
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ViewRule {
    pub read_rule: ReadRule,
    pub write_rule: WriteRule,
    pub read_filters: Vec<FilterRule>,
    pub write_filters: Vec<FilterRule>,
}
