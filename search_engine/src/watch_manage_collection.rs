use dependencies_sync::{
    bson::{from_document, Document},
    log,
    mongodb::{
        change_stream::{
            event::{ChangeStreamEvent, OperationType},
            ChangeStream,
        },
        error::Error,
        options::{ChangeStreamOptions, ReadConcern},
    },
    rust_i18n::{self, t},
    tokio,
    tokio_stream::StreamExt,
};

use database::get_collection_by_id;

use crate::event_handles::handle_insert_event;
use crate::event_handles::{handle_delete_event, handle_update_event};

pub async fn watch_manage_collection(manage_id: i32) {
    tokio::spawn(async move {
        log::info!("{}: {}", t!("开始监听管理集"), manage_id);

        let collection = get_collection_by_id(&manage_id.to_string()).await.unwrap();
        let read_concern = ChangeStreamOptions::builder()
            .read_concern(Some(ReadConcern::majority()))
            .build();
        let mut change_stream: ChangeStream<ChangeStreamEvent<Document>> =
            match collection.watch(None, Some(read_concern)).await {
                Ok(r) => r,
                Err(e) => {
                    log::error!("{}: {}", t!("取得监听数据流发生错误"), manage_id);
                    panic!("{}", e);
                }
            };

        while let result = &change_stream.next().await {
            let change_event: &Result<ChangeStreamEvent<Document>, Error> = match result {
                Some(c) => c,
                None => {
                    log::info!("{}: {}", t!("监听管理集结束"), manage_id);
                    break;
                }
            };

            match change_event {
                Ok(r) => {
                    let entity_id = r
                        .document_key
                        .as_ref()
                        .unwrap()
                        .get_object_id("_id")
                        .unwrap()
                        .to_string();

                    match r.operation_type {
                        OperationType::Insert => {
                            handle_insert_event(
                                manage_id,
                                &entity_id,
                                r.full_document.as_ref().unwrap(),
                            );
                        }
                        OperationType::Update => {
                            handle_update_event(
                                manage_id,
                                &entity_id,
                                &r.update_description.as_ref().unwrap().updated_fields,
                            ).await;
                        }
                        OperationType::Delete => {
                            // println!("{:?}", r.full_document.as_ref());
                            handle_delete_event(manage_id, &entity_id);
                        }
                        OperationType::Replace => {
                            // println!("{:?}", r.full_document.as_ref());
                        }
                        _ => {
                            println!("other operation: {:?}", r.operation_type);
                        }
                    }
                    // println!("{} {}: {:?}", t!("修改发生"), manage_id, r);
                }
                Err(err) => {
                    log::error!("{}: {}", t!("监听数据发生错误"), manage_id);
                }
            }
        }

        log::info!("{}: {}", t!("监听管理集结束"), manage_id);
    });
}
