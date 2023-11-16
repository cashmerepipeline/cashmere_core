#[macro_use]
extern crate rust_i18n;
i18n!("locales");

pub mod view_rules_map;

mod add_query_filters;
mod filter_can_read_fields;

mod can_collection_read;
mod can_collection_write;
mod can_entity_read;
mod can_entity_write;
mod can_field_read;
mod can_field_write;
mod can_manage_read;
mod can_manage_write;

mod enum_filter_rule;
mod enum_read_rule;
mod enum_view_level;
mod enum_view_rule_result;
mod enum_write_rule;

mod get_manage_schema_view;
mod manage_view_claims;

mod view_rule;
mod view_rules;

mod init_view_rules;

pub use add_query_filters::*;
pub use filter_can_read_fields::*;

pub use can_collection_read::*;
pub use can_collection_write::*;
pub use can_entity_read::*;
pub use can_entity_write::*;
pub use can_field_read::*;
pub use can_field_write::*;
pub use can_manage_read::*;
pub use can_manage_write::*;

pub use view_rule::*;
pub use view_rules::*;
pub use enum_filter_rule::*;
pub use enum_read_rule::*;
pub use enum_write_rule::*;
pub use get_manage_schema_view::*;

pub use validate_group::*;
pub mod validates;
mod validate_group;
mod validate_is_owner;
mod validate_view_token;

pub use init_view_rules::init_view_rules;