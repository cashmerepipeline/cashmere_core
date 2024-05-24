use cash_result::{add_call_name_to_chain, OperationResult};
use dependencies_sync::{
    bson::{doc, Document},
    tokio_stream::StreamExt,
};
use majordomo::get_majordomo;

use managers::entity_interface::EntityInterface;

pub async fn get_constants(manage_id: &str) -> Result<Vec<Document>, OperationResult> {
    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(manage_id).unwrap();

    let query_doc = doc! {};

    let result = manager
        .inner.get_entity_stream(query_doc, &[], None, None, 0)
        .await;

    match result {
        Ok(mut entities_iter) => {
            let mut result = vec![];
            while let Some(r) = entities_iter.next().await {
                result.push(r);
            }

            Ok(result)
        }
        Err(e) => Err(add_call_name_to_chain(e, "get_constants".to_string())),
    }
}
