use std::sync::OnceLock;

use crate::general_field_ids::{CREATE_TIMESTAMP_FIELD_ID, ID_FIELD_ID, MODIFY_TIMESTAMP_FIELD_ID};

pub static SYSTEM_FIELDS: OnceLock<Vec<String>> = OnceLock::new();

pub fn get_system_fields() -> &'static Vec<String> {
    SYSTEM_FIELDS.get_or_init(|| {
        vec![
            CREATE_TIMESTAMP_FIELD_ID.to_string(),
            MODIFY_TIMESTAMP_FIELD_ID.to_string(),
            // Add system fields here
        ]
    })
}
