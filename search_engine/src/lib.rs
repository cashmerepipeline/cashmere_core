use dependencies_sync::rust_i18n::{self};
rust_i18n::i18n!("locales");

pub use get_manage_index_writer::*;
pub use get_manage_searcher::*;
pub use get_manage_tantivy_index::*;
pub use manage_index_map::*;
pub use search::*;
pub use spaw_writer_commit_thread::*;

pub use get_manage_tantivy_schema::*;
pub use manage_tantivy_schema_map::*;
pub use register_manage_tantivy_schema::*;

pub use watch_manage_collection::*;

pub use search_engine_configs::*;

mod get_manage_index_writer;
mod get_manage_tantivy_index;
mod init_tantivy_index;
mod manage_index_map;
mod manage_index_writer_map;
mod spaw_writer_commit_thread;

mod get_manage_tantivy_schema;
mod manage_tantivy_schema_map;
mod register_manage_tantivy_schema;

mod get_manage_searcher;
mod search;

mod get_tantivy_index_dir;

pub mod event_handles;
mod watch_manage_collection;

mod search_engine_configs;
