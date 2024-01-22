use cash_result::{add_call_name_to_chain, OperationResult};
use database::get_member_view_name;
use dependencies_sync::{
    bson::{doc, Document},
    tokio::{self, sync::mpsc},
    tokio_stream::{wrappers::ReceiverStream, StreamExt},
};
use manage_define::general_field_names::MEMBER_LOOKUP_FIELD_NAME;

/// zh: 返回
pub async fn lookup_members(
    owner_manage_id: &str,
    owner_entity_id: &str,
    self_manage_id: &str,
    query_doc: &Document,
    sort_doc: &Document,
    unsets: &Vec<String>,
    start_oid: Option<&str>,
    // 相对于start_oid的位置跳过数，不包含start_oid
    skip_count: u32,
) -> Result<ReceiverStream<Document>, OperationResult> {
    let view_name = get_member_view_name(owner_manage_id, owner_entity_id, self_manage_id);
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
        &unsets,
        Some(sorts),
        start_oid,
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
