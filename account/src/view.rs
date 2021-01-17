/*
Project: cashmere_server
Creator: 闫刚
Create time: 2020-10-11 16:45
Introduction:
*/


use std::collections::HashMap;

enum FieldVisibility {
    NotVisible,
    Read,
    ReadWrite,
}

enum EntityVisibility{
    All,
    Group,
    Own,
}

struct View{
    id: String,
    manage_id: String,
    account_id: String,
    add_group_ids: Vec<String>,
    or_group_ids: Vec<String>,
}

pub(crate) struct ViewRules{
    manage_rule: HashMap<String, FieldVisibility>,
    collection_rule: HashMap<String, EntityVisibility>,
    schema_rule: HashMap<String, FieldVisibility>,
}