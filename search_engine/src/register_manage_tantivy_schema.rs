use std::sync::Arc;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tantivy::schema::Schema;
use dependencies_sync::log::info;


use super::get_manage_tantivy_schema_map;

pub fn register_manage_tantivy_schema(manage_id: i32, schema: Schema){
    let schema_map_arc = get_manage_tantivy_schema_map();
    let mut schema_map = schema_map_arc.write();
    
    info!("{}: {}", t!("注册索引模式"), manage_id);
    
    schema_map.insert(manage_id, Arc::new(schema));
}
