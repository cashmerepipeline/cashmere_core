use dependencies_sync::rust_i18n::{self};
rust_i18n::i18n!("locales");

pub use get_manage_index_writer::*;
pub use get_manage_searcher::*;
pub use get_manage_tantivy_index::*;
pub use manage_index_map::*;
pub use search::*;
pub use spaw_writer_commit_thread::*;
pub use spaw_commit_thread::*;

pub use get_manage_tantivy_schema::*;
pub use tantivy_schema::*;
pub use tantivy_index::*;
pub use tantivy_writer::*;
pub use manage_tantivy_schema_map::*;
pub use register_manage_tantivy_schema::*;

pub use watch_database::*;
pub use watch_manage_collection::*;

pub use search_engine_configs::*;
pub use init_single_tantivy_index::*;
pub use init_search_engine::*;

mod search_engine_runtime;
mod get_manage_index_writer;
mod get_manage_tantivy_index;
mod init_single_tantivy_index;
mod manage_index_map;
mod manage_index_writer_map;
mod spaw_writer_commit_thread;
mod spaw_commit_thread;

mod get_manage_tantivy_schema;
mod manage_tantivy_schema_map;
mod register_manage_tantivy_schema;
mod tantivy_schema;
mod tantivy_index;
mod tantivy_writer;
mod tantivy_searcher;

mod get_manage_searcher;
mod search;

mod get_tantivy_index_dir;

pub mod collection_event_handles;
pub mod database_event_handles;
mod watch_manage_collection;
mod watch_database;

mod search_engine_configs;

mod get_text_options;
mod get_tokenizers;

mod init_search_engine;