use cash_result::{add_call_name_to_chain, OperationResult};
use dependencies_sync::{bson::Document, tokio::{self, sync::mpsc}, tokio_stream::{wrappers::ReceiverStream, StreamExt}};
use entity::hard_code_cache::hard_coded_cache_get_entity_stream;


pub async fn get_entity_stream(
    manage_id: &'static str,
    matche_doc: Document,
    unsets: &[String],
    sorts: Option<Document>,
    start_oid: Option<&str>,
    skip_count: u32,
    hard_coded: bool,
) -> Result<ReceiverStream<Document>, OperationResult> {
    if hard_coded {
        return Ok(hard_coded_cache_get_entity_stream(manage_id).await);
    }

    match entity::get_query_cursor(manage_id, matche_doc, unsets, sorts, start_oid, skip_count)
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
        Err(e) => Err(add_call_name_to_chain(e, "get_entity_stream".to_string())),
    }
}
