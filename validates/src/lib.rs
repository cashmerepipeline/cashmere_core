use dependencies_sync::rust_i18n::{self, i18n};
i18n!("locales");

pub use validate_auth_token::*;
pub use validate_has_role_group::*;

pub use validate_manage_id::*;
pub use validate_manage_hard_coded::*;

pub use validate_entity_id::*;
pub use validate_field_id::*;

pub use validate_name::*;
pub use validate_value_doc::*;
pub use validate_role_group::*;
pub use validate_is_login::*;
pub use validate_description_length::*;

pub use validate_bson_document_bytes::*;

pub use validate_data_field::*;
pub use validate_data_fields::*;

mod validate_auth_token;
mod validate_has_role_group;

mod validate_manage_id;
mod validate_manage_hard_coded;

mod validate_entity_id;

mod validate_field_id;

mod validate_name;
mod validate_value_doc;
mod validate_role_group;
mod validate_is_login;
mod validate_description_length;

mod validate_bson_document_bytes;
mod validate_data_field;
mod validate_data_fields;
