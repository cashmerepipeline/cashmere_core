/*
使用单例模式创建数据库 client
所有操作只使用一个client, 需要进一步测试
*/

use dependencies_sync::rust_i18n::{self};
rust_i18n::i18n!("locales");

pub use collection_exists::*;
pub use get_collection_by_id::*;
pub use get_database::*;
pub use get_database_name::*;
pub use get_mongodb_client::*;

pub use get_member_db_view_name::*;
pub use get_member_view::*;

pub use database_configs::*;
pub use get_ids_collection::*;
pub use get_manages_collection::*;
pub use init_ids_count_field::*;

mod collection_exists;
mod get_collection_by_id;
mod get_database;
mod get_database_name;
mod get_mongodb_client;

mod get_member_db_view_name;
mod get_member_view;

mod database_configs;
mod get_ids_collection;
mod get_manages_collection;
mod init_ids_count_field;
