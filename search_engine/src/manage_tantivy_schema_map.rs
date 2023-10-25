use dependencies_sync::tantivy::schema::Schema;
use dependencies_sync::parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

pub type ManageSchemaMap = HashMap<i32, Arc<Schema>>;

static mut MANAGE_TANTIVY_SCHEMA_MAP: Option<Arc<RwLock<ManageSchemaMap>>> = None;

pub fn get_manage_tantivy_schema_map() -> Arc<RwLock<ManageSchemaMap>> {
  unsafe {

    let map = MANAGE_TANTIVY_SCHEMA_MAP.get_or_insert_with(|| {
      Arc::new(RwLock::new(HashMap::new()))
    });

    map.clone()
  }
}


