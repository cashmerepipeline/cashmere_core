use serde::{Deserialize, Serialize};

use dependencies_sync::bson::Timestamp;
use dependencies_sync::linked_hash_map::LinkedHashMap;

use crate::SchemaField;

/// zh: 管理定义，管理实体具有大部分实体属性，在数据库中有具体的实体。管理相关的操作在管理器中定义，每个管理对应一个管理器。
/// en:
#[derive(Debug, Serialize, Deserialize)]
pub struct Manage {
    // zh: 数据库分配的id
    pub object_id: String,
    // zh: 管理id
    pub id: i32,
    // zh: 命名表，{lang:native_name, ...}
    pub name_map: LinkedHashMap<String, String>,
    // zh: 创建人
    pub creator: String,
    // zh: 创建时间戳
    pub create_timestamp: Timestamp,
    // zh: 修改人
    pub modifier: String,
    // zh: 修改时间戳
    pub modify_timestamp: Timestamp,

    // zh: 所属人
    pub owner: String,
    // zh: 所属组
    pub groups: Vec<String>,

    // zh: 实体模式表
    // en: schema
    pub schema: Vec<SchemaField>,

    // zh: 能被搜索
    //  en: is_searchable
    pub is_searchable: bool,

    // zh: 注释
    // en: description
    pub description: String,
}
