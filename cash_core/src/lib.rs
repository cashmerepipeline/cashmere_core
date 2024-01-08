use dependencies_sync::once_cell;
use dependencies_sync::rust_i18n::{self, i18n};
i18n!("locales");

pub use manage::*;
pub use manage_from_document::*;

pub use schema_field::*;
pub use general_schema_fields::*;
pub use schema_field_exists::*;

mod general_schema_fields;
mod schema_field;
mod schema_field_exists;

mod rang;

mod manage;
mod manage_from_document;
