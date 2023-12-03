use dependencies_sync::rust_i18n::{self, i18n};
i18n!("locales");

pub use request_account_context::*;
pub use get_manage_schema_fields::*;

mod request_account_context;
mod get_manage_schema_fields;