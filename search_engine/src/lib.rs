// #[macro_use]
// extern crate tantivy;

use dependencies_sync::rust_i18n::{self};
rust_i18n::i18n!("locales");

pub use search::*;

pub use tantivy_schema::*;
pub use tantivy_index::*;
pub use tantivy_writer::*;

pub use watch_database::*;

pub use search_engine_configs::*;
pub use init_tantivy_index::*;
pub use init_search_engine::*;

mod search_engine_runtime;
mod init_tantivy_index;

mod tantivy_schema;
mod tantivy_index;
mod tantivy_writer;
mod tantivy_searcher;

mod search;

pub mod database_event_handles;

mod watch_database;
mod is_searchable;

mod spaw_commit_thread;

mod search_engine_configs;

mod get_text_options;
mod get_tokenizers;

mod init_search_engine;