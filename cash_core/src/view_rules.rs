/*
Author: 闫刚 (yes7rose@sina.com)
view_rules.rs (c) 2020
Desc: 映像规则
Created:  2020-11-23T15:56:30.639Z
Modified: !date!
*/

use std::fmt::{self, Display, Formatter};
use linked_hash_map::LinkedHashMap;
use serde::{Deserialize, Serialize};

/// 映像结果
enum ViewRuleResult {
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


// 映像规则层级
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ViewRuleLevel{
    Manage,
    Entity,
    Schema
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ReadRule {
    // 不可读
    InVisible,
    // 全部可读
    Read,
    // 只组的 可读
    GroupRead,
    // 只主的 可读
    OwnerRead,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum WriteRule {
    // 只读
    InVisible,
    // 全部可写
    Write,
    // 只组的可写
    GroupWrite,
    // 只主的 可写
    OwnerWrite,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Filter {
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

/// 映像
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ViewRule {
    pub read_rule: ReadRule,
    pub write_rule: WriteRule,
    pub read_filters: Vec<Filter>,
    pub write_filters: Vec<Filter>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ViewRules {
    // {组：规则}
    pub manage: LinkedHashMap<String, ViewRule>,
    pub entity: LinkedHashMap<String, ViewRule>,
    // {属性：组：规则}
    pub schema: LinkedHashMap<String, LinkedHashMap<String, ViewRule>>,
}

pub type ViewRulesMap = LinkedHashMap<String, ViewRules>;

// 层级
impl Display for ViewRuleLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ViewRuleLevel::Manage => write!(f, "Manage"),
            ViewRuleLevel::Entity => write!(f, "Entity"),
            ViewRuleLevel::Schema => write!(f, "Schema"),
        }
    }
}