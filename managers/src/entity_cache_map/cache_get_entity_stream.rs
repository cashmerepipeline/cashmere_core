use std::ops::Deref;
use std::sync::Arc;

use dependencies_sync::{tokio, tokio_stream};
use dependencies_sync::bson::Document;
use dependencies_sync::futures::StreamExt;
use dependencies_sync::tokio::sync::mpsc;
use dependencies_sync::tokio_stream::wrappers::ReceiverStream;

use crate::entity_cache_map::get_manage_entity_cache;

pub async fn cache_get_entity_stream(manage_id: &'static str) -> ReceiverStream<Document> {
    let entities = {
        let c_map = get_manage_entity_cache(manage_id);
        let e_map = c_map.read();
        e_map
            .values().cloned()
            .collect::<Vec<Arc<Document>>>()
    };

    let (tx, rx) = mpsc::channel(1);
    tokio::spawn(async move {
        let mut entity_stream = tokio_stream::iter(entities);
        while let Some(r) = &entity_stream.next().await {
            tx.send(r.deref().clone()).await.unwrap();
        }
    });

    ReceiverStream::new(rx)
}
