use cash_result::{add_call_name_to_chain, operation_failed, OperationResult};
use database::{get_database, get_member_view_name};
use dependencies_sync::{
    bson::{doc, Document},
    log,
    mongodb::{
        options::{Acknowledgment, ReadConcern, TransactionOptions, WriteConcern},
        Collection,
    },
    rust_i18n::{self, t}, tokio_stream::{wrappers::ReceiverStream, StreamExt}, tokio::{sync::mpsc, self},
};
use manage_define::{
    field_ids::{
        MEMBERS_OWNER_ENTITY_ID_FIELD_ID, MEMBERS_OWNER_MANAGE_ID_FIELD_ID,
        MEMBERS_SELF_ENTITY_ID_FIELD_ID, MEMBERS_SELF_MANAGE_ID_FIELD_ID,
    },
    general_field_ids::*,
    general_field_names::MEMBER_LOOKUP_FIELD_NAME,
    manage_ids::MEMBERS_MANAGE_ID,
};

use crate::entity_cache_map::cache_update_entity;

/// zh: 返回
pub async fn lookup_members(
    owner_manage_id: &i32,
    owner_entity_id: &str,
    self_manage_id: &i32,
    query_doc: &Document,
    sort_doc: &Document,
    unsets: &Vec<String>,
    start_oid: Option<&str>,
    skip_count: u32,
) -> Result<ReceiverStream<Document>, OperationResult> {
    let view_name = get_member_view_name(&owner_manage_id.to_string(), owner_entity_id, &self_manage_id.to_string());
    let mut match_doc = doc! {};
    query_doc.iter().for_each(|(k, v)| {
        match_doc.insert(
            format!("{}.{}", MEMBER_LOOKUP_FIELD_NAME, k.clone()),
            v.clone(),
        );
    });
    let mut sorts = doc! {};
    sort_doc.iter().for_each(|(k, v)| {
        sorts.insert(
            format!("{}.{}", MEMBER_LOOKUP_FIELD_NAME, k.clone()),
            v.clone(),
        );
    });
    let unsets = unsets
        .iter()
        .map(|s| format!("{}.{}", MEMBER_LOOKUP_FIELD_NAME, s.clone()))
        .collect::<Vec<String>>();

    match entity::get_query_cursor(
        &view_name,
        match_doc,
        Some(unsets),
        Some(sorts),
        start_oid.as_deref(),
        skip_count,
    )
    .await
    {
        Ok(mut r) => {
            let (tx, rv) = mpsc::channel(1);
            tokio::spawn(async move {
                while let Some(r) = r.next().await {
                    let _ = tx.send(r.unwrap()).await;
                }
            });

            Ok(ReceiverStream::new(rv))
        }
        Err(e) => Err(add_call_name_to_chain(
            e,
            "manager::get_entity_stream".to_string(),
        )),
    }
}
