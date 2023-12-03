use dependencies_sync::{
    bson::Document,
    log,
    mongodb::{
        change_stream::{
            event::{ChangeStreamEvent, OperationType},
            ChangeStream,
        },
        options::{ChangeStreamOptions, FullDocumentType, ReadConcern},
    },
    rust_i18n::{self, t},
    tokio_stream::StreamExt,
};

use database::get_database;
use majordomo::get_majordomo;
use managers::ManagerTrait;

use crate::database_event_handles::{handle_delete_event, handle_update_event};
use crate::{
    database_event_handles::handle_insert_event, search_engine_runtime::get_search_engine_runtime,
};

pub async fn watch_database() {
    let run_time = get_search_engine_runtime();

    run_time.spawn(async move {
        log::info!("{}", t!("开始监听数据库"));

        let database = get_database().await;
        let read_concern = ChangeStreamOptions::builder()
            .read_concern(Some(ReadConcern::majority()))
            .full_document(Some(FullDocumentType::UpdateLookup))
            .build();

        let mut change_stream: ChangeStream<ChangeStreamEvent<Document>> =
            match database.watch(None, Some(read_concern)).await {
                Ok(r) => r,
                Err(e) => {
                    log::error!("{}", t!("取得监听数据流发生错误"));
                    panic!("{}", e);
                }
            };

        // while let result = &change_stream.next().await {
        //     let change_event: &Result<ChangeStreamEvent<Document>, Error> = match result {
        //         Some(c) => c,
        //         None => {
        //             log::info!("{}: {}", t!("监听管理集结束"), manage_id);
        //             break;
        //         }
        //     };

        while let Some(change_event) = &change_stream.next().await {
            match change_event {
                Ok(r) => {
                    let manage_id = if let Some(ref n) = r.ns {
                        if let Some(ref c) = n.coll {
                            c.parse::<i32>().unwrap()
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    };

                    {
                        // 判断是否需要搜索
                        let majordomo_arc = get_majordomo();
                        let manager = majordomo_arc.get_manager_by_id(manage_id).unwrap();
                        if !manager.is_searchable().await {
                            continue;
                        }
                    }
                    log::info!("{}: {}", t!("接收到数据库变更"), manage_id);

                    let _object_id = if let Some(ref dk) = r.document_key {
                        if let Ok(ref oid) = dk.get_object_id("_id") {
                            oid.to_string()
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    };

                    let object_id = r
                        .document_key
                        .as_ref()
                        .unwrap()
                        .get_object_id("_id")
                        .unwrap()
                        .to_string();

                    match r.operation_type {
                        OperationType::Insert => {
                            handle_insert_event(manage_id, r.full_document.as_ref().unwrap());
                        }
                        OperationType::Update => {
                            handle_update_event(
                                manage_id,
                                &object_id,
                                &r.update_description.as_ref().unwrap().updated_fields,
                                r.full_document.as_ref().unwrap(),
                            );
                        }
                        OperationType::Delete => {
                            // println!("{:?}", r.full_document.as_ref());
                            handle_delete_event(manage_id, &object_id);
                        }
                        OperationType::Replace => {
                            // println!("{:?}", r.full_document.as_ref());
                        }
                        _ => {
                            log::debug!("{}: {:?}", t!("发生其他变更"), r.operation_type);
                        }
                    }
                    // println!("{} {}: {:?}", t!("修改发生"), manage_id, r);
                }
                Err(err) => {
                    log::error!("{}: {}", t!("监听数据发生错误"), err);
                }
            }
        }

        log::info!("{}", t!("监听管理集结束"));
    });
}
