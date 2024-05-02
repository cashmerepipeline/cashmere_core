use crate::{collection_exists, get_database, get_member_view_name};
use dependencies_sync::bson::{doc, Document};
use dependencies_sync::mongodb::options::CreateCollectionOptions;
use dependencies_sync::mongodb::Collection;
use manage_define::field_ids::{
    MEMBERS_OWNER_ENTITY_ID_FIELD_ID, MEMBERS_OWNER_MANAGE_ID_FIELD_ID,
    MEMBERS_SELF_ENTITY_ID_FIELD_ID, MEMBERS_SELF_MANAGE_ID_FIELD_ID,
};
use manage_define::general_field_ids::ID_FIELD_ID;
use manage_define::hard_coded_field_names::MEMBER_LOOKUP_FIELD_NAME;

/// 取得集合
pub async fn get_member_view(
    owner_manage_id: &str,
    owner_entity_id: &str,
    self_manage_id: &str,
) -> Option<Collection<Document>> {
    let cashmere_db = get_database().await;
    let view_name = get_member_view_name(owner_manage_id, owner_entity_id, self_manage_id);

    // zh: 不存在，新建视图
    if !collection_exists::collection_exists(&view_name).await {
        let mut match_doc = doc! {};
        match_doc.insert(
            MEMBERS_OWNER_MANAGE_ID_FIELD_ID.to_string(),
            owner_manage_id,
        );
        match_doc.insert(
            MEMBERS_OWNER_ENTITY_ID_FIELD_ID.to_string(),
            owner_entity_id,
        );
        match_doc.insert(MEMBERS_SELF_MANAGE_ID_FIELD_ID.to_string(), self_manage_id);
        let match_doc = doc! {"$match": match_doc};

        let mut lookup_doc = doc! {};
        lookup_doc.insert("from", self_manage_id);
        lookup_doc.insert("localField", MEMBERS_SELF_ENTITY_ID_FIELD_ID.to_string());
        lookup_doc.insert("foreignField", ID_FIELD_ID.to_string());
        lookup_doc.insert("as", MEMBER_LOOKUP_FIELD_NAME);
        let lookup_doc = doc! {"$lookup": lookup_doc};

        let unwind_doc = doc! {"$unwind": format!("${}", MEMBER_LOOKUP_FIELD_NAME)};

        let create_options = CreateCollectionOptions::builder()
            .view_on(Some(owner_manage_id.to_string()))
            .pipeline(vec![match_doc, lookup_doc, unwind_doc])
            .build();

        cashmere_db.create_collection(view_name.clone(), Some(create_options));
    }

    Some(cashmere_db.collection(&view_name))
}
