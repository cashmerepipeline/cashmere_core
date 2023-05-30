use dependencies_sync::linked_hash_map::LinkedHashMap;
use serde::{Deserialize, Serialize};

use super::view_rule::ViewRule;

/// 映像规则
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ViewRules {
    // 管理级，{组：规则}
    pub manage: LinkedHashMap<String, ViewRule>,
    // 集合级，{组：规则}
    pub collection: LinkedHashMap<String, ViewRule>,
    // {属性：组：规则}
    pub schema: LinkedHashMap<String, LinkedHashMap<String, ViewRule>>,
}
