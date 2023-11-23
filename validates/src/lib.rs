use dependencies_sync::rust_i18n::{self, i18n};
i18n!("locales");

pub use validate_manage_id::*;
pub use validate_entity_id::*;
pub use validate_name::*;
pub use validate_field_id::*;
pub use validate_value_doc::*;
pub use validate_role_group::*;
pub use validate_is_login::*;
pub use validate_description_length::*;

mod validate_manage_id;
mod validate_entity_id;
mod validate_name;
mod validate_field_id;
mod validate_value_doc;
mod validate_role_group;
mod validate_is_login;
mod validate_description_length;