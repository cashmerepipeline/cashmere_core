use dependencies_sync::rust_i18n::{self, i18n};
i18n!("locales");

pub use get_id::*;
pub use get_init_items::*;
pub use get_init_view_rules::*;
pub use get_name_map::*;
pub use get_schema::*;
pub use get_is_searchable::*;
pub use get_toml_files_of_dir::*;
pub use get_toml_map::*;
pub use get_tomls_from_pathes::*;
pub use generate_manage_defines::*;

mod get_id;
mod get_name_map;
mod get_schema;
mod get_is_searchable;
mod get_init_items;
mod get_init_view_rules;
mod get_toml_files_of_dir;
mod get_tomls_from_pathes;
mod get_toml_map;
mod generate_manage_defines;
