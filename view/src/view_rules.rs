use dependencies_sync::{indexmap::IndexMap};
use serde::{Deserialize, Serialize};

use super::view_rule::ViewRule;

/// 映像规则
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ViewRules {
    // 管理级，{组：规则}
    pub manage: IndexMap<String, ViewRule>,
    // 集合级，{组：规则}
    pub collection: IndexMap<String, ViewRule>,
    // {属性：组：规则}
    pub schema: IndexMap<String, IndexMap<String, ViewRule>>,
}
