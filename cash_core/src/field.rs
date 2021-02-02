/*
Author: 闫刚 (yes7rose@sina.com)
field.rs (c) 2020
Desc: 属性
Created:  2020-11-26T12:47:25.692Z
Modified: !date!
*/

use std::{fmt::{Display, Formatter}};

use linked_hash_map::LinkedHashMap;
use serde::{Serialize, Deserialize};
use std::str::FromStr;
use toml::map::Map;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum FieldDataType {
    Bool,
    Int32,
    Int64,
    UInt32,
    UInt64,
    F32,
    F64,
    String,
    List,
    Bytes,
    Map,
    Date,
    DateTime,
    None,
}

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum Language {
    Cn,
    En,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Name {
    language: Language,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PropertyField {
    pub id: i32,
    pub name: LinkedHashMap<String, String>,
    pub data_type: FieldDataType,
    pub removed: bool,
}

impl Display for FieldDataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldDataType::Bool => write!(f, "Bool"),
            FieldDataType::Int32 => write!(f, "Int32"),
            FieldDataType::Int64 => write!(f, "Int64"),
            FieldDataType::UInt32 => write!(f, "UInt32"),
            FieldDataType::UInt64 => write!(f, "UInt64"),
            FieldDataType::F32 => write!(f, "F32"),
            FieldDataType::F64 => write!(f, "F64"),
            FieldDataType::String => write!(f, "String"),
            FieldDataType::List => write!(f, "List"),
            FieldDataType::Bytes => write!(f, "Bytes"),
            FieldDataType::Map => write!(f, "Map"),
            FieldDataType::Date => write!(f, "Date"),
            FieldDataType::DateTime => write!(f, "DateTime"),
            FieldDataType::None => write!(f, "None"),
        }
    }
}

impl From<String> for FieldDataType {
    fn from(s: String) -> Self {
        if s == "Bool".to_string() { return FieldDataType::Bool; } else if s == "Int32".to_string() { return FieldDataType::Bool; } else if s == "Int64".to_string() { return FieldDataType::Bool; } else if s == "UInt32".to_string() { return FieldDataType::Bool; } else if s == "UInt64".to_string() { return FieldDataType::Bool; } else if s == "F32".to_string() { return FieldDataType::Bool; } else if s == "F64".to_string() { return FieldDataType::Bool; } else if s == "String".to_string() { return FieldDataType::Bool; } else if s == "List".to_string() { return FieldDataType::Bool; } else if s == "Bytes".to_string() { return FieldDataType::Bool; } else if s == "Map".to_string() { return FieldDataType::Bool; } else if s == "Date".to_string() { return FieldDataType::Bool; } else if s == "DateTime".to_string() { return FieldDataType::Bool; } else { return FieldDataType::None; }
    }
}


impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Cn => write!(f, "Cn"),
            Language::En => write!(f, "En"),
        }
    }
}

impl PropertyField {
    pub fn has_name(&self, name: &String) -> bool {
        let names: Vec<String> =
            self.name.values()
                .map(|v| v.clone())
                .collect();

        names.contains(name)
    }

    pub fn from_toml(toml: &toml::map::Map<String, toml::Value>) -> PropertyField {
        let id = toml.get("id").unwrap().as_integer().unwrap() as i32;
        let name: LinkedHashMap<String, String> = toml::from_str(&toml.get("name").unwrap().to_string()).unwrap();
        // println!("{:?}", name);
        let data_type: FieldDataType = toml::from_str(&toml.get("data_type").unwrap().to_string()).unwrap();
        let removed: bool = toml.get("removed").unwrap().as_bool().unwrap();

        PropertyField {
            id: id,
            name: name,
            data_type: data_type,
            removed: removed,
        }
    }

    pub fn from_bson(doc: &bson::Document) -> PropertyField {
        let id = doc.get_i32("id").unwrap();
        let name: LinkedHashMap<String, String> = bson::from_document(doc.get_document("name").unwrap().clone()).unwrap();
        // println!("{:?}", name);
        let data_type: FieldDataType = bson::from_bson(doc.get("data_type").unwrap().clone()).unwrap();
        let removed: bool = doc.get_bool("removed").unwrap();

        PropertyField {
            id: id,
            name: name,
            data_type: data_type,
            removed: removed,
        }
    }
}

pub mod ids {
    // ----------------
    // 必须属性
    // ----------------
    pub const ID_FIELD_ID: i32 = 1000;
    pub const NAME_FIELD_ID: i32 = 1001;
    pub const CREATOR_FIELD_ID: i32 = 1002;
    pub const CREATE_TIMESTAMP_FIELD_ID: i32 = 1003;
    pub const MODIFIER_FIELD_ID: i32 = 1004;
    pub const MODIFY_TIMESTAMP_FIELD_ID: i32 = 1005;
    pub const OWNER_FIELD_ID: i32 = 1006;
    pub const GROUPS_FIELD_ID: i32 = 1007;
    pub const DATAS_FIELD_ID: i32 = 1008;
    pub const COMMENTS_FIELD_ID: i32 = 1009;

    // 特殊的描写属性
    pub const ACCOUNTS_PASSWORD_FIELD_ID: i32 = 2010;
    pub const ACCOUNTS_LOGIN_TIMESTAMPS_FIELD_ID: i32 = 2012;

    pub const MANAGES_SCHEMA_FIELD_ID: i32 = 2008;

    pub const TEMPLATES_MANAGE_FIELD_ID: i32 = 2008;
    pub const TEMPLATES_PRESETS_FIELD_ID: i32 = 2008;

    pub const VIEW_RULES_MANAGE_FIELD_ID: i32 = 2008;
    pub const VIEW_RULES_COLLECTION_FIELD_ID: i32 = 2009;
    pub const VIEW_RULES_ENTITY_FIELD_ID: i32 = 2010;

    pub const WORK_MANAGE_FIELD_ID: i32 = 2008;
    pub const WORK_PROCEDURE_FIELD_ID: i32 = 2009;
    pub const WORK_WORK_PHASES_FIELD_ID: i32 = 2010;

    pub const WORK_PHASE_PHASES_FIELD_ID: i32 = 2008;

    pub const PROCEDURE_WORK_ID_FIELD_ID: i32 = 2008;
    pub const PROCEDURE_PHASE_NAME_FIELD_ID: i32 = 2009;
    pub const PROCEDURE_WORK_NODES_FIELD_ID: i32 = 2010;
    pub const PROCEDURE_LINKS_FIELD_ID: i32 = 2010;

    pub const WORK_NODE_PROCEDURE_FIELD_ID: i32 = 2008;
    pub const WORK_NODE_TASKS_FIELD_ID: i32 = 2009;
    pub const WORK_NODE_WORKER_FIELD_ID: i32 = 2010;
    pub const WORK_NODE_REFERENCE_DATAS_FIELD_ID: i32 = 2009;
    pub const WORK_NODE_DEPENDED_DATAS_FIELD_ID: i32 = 2011;
    pub const WORK_NODE_WORK_DATAS_FIELD_ID: i32 = 2011;
    pub const WORK_NODE_OUT_DATAS_FIELD_ID: i32 = 2011;

    pub const TASK_WORK_NODE_FIELD_ID: i32 = 2008;
    pub const TASK_STATUS_FIELD_ID: i32 = 2009;
    pub const TASK_DATA_FIELD_ID: i32 = 2010;

    pub const DATA_TASK_FIELD_ID: i32 = 2010;

    //------图知-------
    pub const GRAPHS_EDGES_FIELD_ID:i32=2008;
    pub const GRAPHS_INDIVIDUAL_LEVEL_FIELD_ID:i32=2009;
    pub const GRAPHS_INDIVIDUAL_ID_FIELD_ID:i32=2010;
    pub const GRAPHS_DOMAIN_ID_FIELD_ID:i32=2011;
    pub const GRAPHS_COMPOSE_TYPE_FIELD_ID:i32=2012;
    pub const GRAPHS_QUESTIONS_FIELD_ID:i32=2013;

    pub const ROADMAPS_GRAPH_FIELD_ID:i32=2008;
    pub const ROADMAPS_TAG_TYPES_FIELD_ID:i32=2009;
    pub const ROADMAPS_VIS_PERSONS_FIELD_ID:i32=2010;
    pub const ROADMAPS_VIS_CLASSES_FIELD_ID:i32=2011;
    pub const ROADMAPS_VIS_GROUPS_FIELD_ID:i32=2012;
    pub const ROADMAPS_VIS_DEPARTMENT_FIELD_ID:i32=2013;
    pub const ROADMAPS_VIS_ORGANIZATIONS_FIELD_ID:i32=2014;
    pub const ROADMAPS_IS_PUBLIC_FIELD_ID:i32=2015;

    pub const TAG_TYPES_DOMAIN_FIELD_ID:i32 = 2008;
    pub const TAG_TYPES_DATA_FIELDS_FIELD_ID:i32 = 2009;
    pub const TAG_TYPES_DESCTIPTION_FIELD_ID:i32 = 2010;

    pub const TAG_MAPS_ROADMAP_FIELD_ID:i32=2008;
    pub const TAG_MAPS_TAGS_FIELD_ID:i32=2009;
    pub const TAG_MAPS_VIS_PERSONS_FIELD_ID:i32=2010;
    pub const TAG_MAPS_VIS_CLASSES_FIELD_ID:i32=2011;
    pub const TAG_MAPS_VIS_GROUPS_FIELD_ID:i32=2012;
    pub const TAG_MAPS_VIS_DEPARTMENT_FIELD_ID:i32=2013;
    pub const TAG_MAPS_VIS_ORGANIZATIONS_FIELD_ID:i32=2014;
    pub const TAG_MAPS_IS_PUBLIC_FIELD_ID:i32=2015;

    pub const COMMENTS_TARGET_MANAGE_ID_FIELD_ID:i32=2008;
    pub const COMMENTS_TARGET_ENTITY_ID_FIELD_ID:i32=2009;
    pub const COMMENTS_CONTENTS_FIELD_ID:i32=2010;

    pub const QUESTIONS_POINTS_FIELD_ID:i32=2008;
    pub const QUESTIONS_ANSWERS_FIELD_ID:i32=2009;
    pub const QUESTIONS_CONTENTS_FIELD_ID:i32=2010;

    pub const ANSWERS_QUESTION_FIELD_ID:i32=2008;
    pub const ANSWERS_CONTENTS_FIELD_ID:i32=2009;
}