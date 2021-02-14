/*
Author: 闫刚 (yes7rose@sina.com)
schema.rs (c) 2020
Desc: 描写管理
Created:  2020-12-18T04:00:07.277Z
Modified: !date!
*/

use property_field::PropertyField;

pub fn schema_field_exists(field_id: i32, schema: &Vec<PropertyField>) -> bool{
    let field_ids: Vec<i32> = schema.iter().map(|x| x.id.clone()).collect();

    field_ids.contains(&field_id)
}

pub fn new_schema_field_update_cache(){}

pub fn new_shema_field_update_doc_cache() {}

pub fn new_schema_field_update_database() {}