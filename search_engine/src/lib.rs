use dependencies_sync::rust_i18n::{self};
rust_i18n::i18n!("locales");

pub use manage_index_map::*;
pub use get_manage_tantivy_index::*;

pub use get_manage_searcher::*;
pub use search::*;

pub use manage_tantivy_schema_map::*;
pub use get_manage_tantivy_schema::*;
pub use register_manage_tantivy_schema::*;

pub use watch_manage_collection::*;

mod manage_index_map;
mod init_tantivy_index;
mod get_manage_tantivy_index;

mod manage_tantivy_schema_map;
mod get_manage_tantivy_schema;
mod register_manage_tantivy_schema;

mod get_manage_searcher;
mod search;

mod get_tantivy_index_dir;

mod watch_manage_collection;
pub mod event_handles;