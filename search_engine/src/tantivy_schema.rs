use std::sync::Arc;

use dependencies_sync::tantivy::schema::{Schema, STORED, FAST, TEXT};
use manage_define::general_field_ids::*;

use crate::get_text_options::get_text_options;

static mut TANTIVY_SCHEMA_MAP: Option<Arc<Schema>> = None;

pub fn get_tantivy_schema() -> Arc<Schema> {
    unsafe {
        if TANTIVY_SCHEMA_MAP.is_none() {
            TANTIVY_SCHEMA_MAP = Some(Arc::new(init_schema()));
        }
        TANTIVY_SCHEMA_MAP.as_ref().unwrap().clone()
    }
}

fn init_schema() -> Schema {
    let text_options = get_text_options();

    let mut schema_builder = Schema::builder();
    let _mid = schema_builder.add_u64_field("mid", STORED | FAST);
    let _id = schema_builder.add_text_field("_id", STORED | TEXT);
    let _idf = schema_builder.add_text_field(ID_FIELD_ID.to_string().as_ref(), STORED | TEXT);
    let _name_map =
        schema_builder.add_json_field(NAME_MAP_FIELD_ID.to_string().as_ref(), text_options.clone());
    let _description =
        schema_builder.add_text_field(DESCRIPTIONS_FIELD_ID.to_string().as_ref(), text_options);
    let _modify_time = schema_builder.add_u64_field(
        MODIFY_TIMESTAMP_FIELD_ID.to_string().as_ref(),
        STORED | FAST,
    );

    schema_builder.build()
}
