use crate::general_schema_fields;
use crate::Manage;
use crate::SchemaField;
use cash_result::OperationResult;
use dependencies_sync::bson::{self, Document};
use manage_define::field_ids::MANAGES_SCHEMA_FIELD_ID;
use manage_define::general_field_ids::{
    CREATE_TIMESTAMP_FIELD_ID, CREATOR_FIELD_ID, DESCRIPTION_FIELD_ID, GROUPS_FIELD_ID,
    ID_FIELD_ID, IS_SEARCHABLE_FIELD_ID, MODIFIER_FIELD_ID, MODIFY_TIMESTAMP_FIELD_ID,
    NAME_MAP_FIELD_ID, OWNER_FIELD_ID,
};

/// bson文档-->管理实体
pub fn manage_from_document(manage_doc: Document) -> Result<Manage, OperationResult> {
    let object_id = manage_doc.get_str("_id").unwrap();
    let id: String = manage_doc
        .get_str(&ID_FIELD_ID.to_string())
        .unwrap()
        .to_string();

    let name_map = bson::from_document(
        manage_doc
            .get_document(&NAME_MAP_FIELD_ID.to_string())
            .unwrap()
            .clone(),
    )
    .unwrap();

    let creator = manage_doc.get_str(&CREATOR_FIELD_ID.to_string()).unwrap();
    let create_timestamp = manage_doc
        .get_timestamp(CREATE_TIMESTAMP_FIELD_ID.to_string())
        .unwrap();

    let modifier = manage_doc.get_str(&MODIFIER_FIELD_ID.to_string()).unwrap();
    let modify_timestamp = manage_doc
        .get_timestamp(MODIFY_TIMESTAMP_FIELD_ID.to_string())
        .unwrap();

    let owner = manage_doc.get_str(&OWNER_FIELD_ID.to_string()).unwrap();
    let groups: Vec<String> = manage_doc
        .get_array(&GROUPS_FIELD_ID.to_string())
        .unwrap()
        .iter()
        .map(|x| x.as_str().unwrap().to_string())
        .collect();

    let mut schema = general_schema_fields().clone();
    manage_doc
        .get_array(&MANAGES_SCHEMA_FIELD_ID.to_string())
        .unwrap()
        .iter()
        .for_each(|x| {
            let field: SchemaField = SchemaField::from_document(x.as_document().unwrap());
            schema.push(field);
        });

    let is_searchable = manage_doc
        .get_bool(IS_SEARCHABLE_FIELD_ID.to_string())
        .unwrap();

    let description = manage_doc
        .get_str(&DESCRIPTION_FIELD_ID.to_string())
        .unwrap_or("");

    Ok(Manage {
        object_id: object_id.to_string(),
        id,
        name_map,
        creator: creator.to_string(),
        create_timestamp,
        modifier: modifier.to_string(),
        modify_timestamp,
        owner: owner.to_string(),
        groups,
        schema,
        is_searchable,
        description: description.to_string(),
    })
}
