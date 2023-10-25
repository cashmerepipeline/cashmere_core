use std::sync::Arc;

use dependencies_sync::tantivy::{schema::{Schema, self}};

use crate::get_manage_tantivy_schema_map;

pub fn get_manage_tantivy_schema(manage_id: i32) -> Option<Arc<Schema>> {
  let schema_map_arc = get_manage_tantivy_schema_map();
  let schema_map = schema_map_arc.read();
  let schema = schema_map.get(&manage_id);
  schema.cloned()
}
