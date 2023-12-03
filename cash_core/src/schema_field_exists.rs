use crate::SchemaField;

pub fn schema_field_exists(field_id: i32, schema: &[SchemaField]) -> bool {
    schema.iter().map(|x| x.id).any(|x| x == field_id)
}