use std::sync::Arc;

use dependencies_sync::{bson::Document, indexmap::IndexMap, parking_lot::RwLock};

/// {entity_id: Document}
pub type HardCodedCacheMap<'a> = Arc<RwLock<IndexMap<String, Document>>>;
