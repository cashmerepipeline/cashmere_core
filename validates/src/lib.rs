use dependencies_sync::rust_i18n::{self, i18n};
i18n!("locales");

pub use validate_entity_id::*;
pub use validate_field_id::*;
pub use validate_value_doc::*;
pub use validate_group_id::*;

pub use get_manage_schema::*;

mod validate_entity_id;
mod validate_field_id;
mod validate_value_doc;
mod get_manage_schema;
mod validate_group_id;