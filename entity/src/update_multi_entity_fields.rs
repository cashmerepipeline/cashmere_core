use cash_result::{operation_failed, operation_succeed, OperationResult};
use database::{get_database, get_database_name};
use dependencies_sync::bson::{self, doc, from_bson, from_slice, Bson, Document};
use dependencies_sync::log;
use dependencies_sync::mongodb::error::{
    Error, ErrorKind, Result as DbResult, TRANSIENT_TRANSACTION_ERROR,
    UNKNOWN_TRANSACTION_COMMIT_RESULT,
};
use dependencies_sync::mongodb::options::{
    Acknowledgment, ReadConcern, TransactionOptions, UpdateOptions, WriteConcern,
};
use dependencies_sync::mongodb::{Client, ClientSession, Collection};

use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::client;
use manage_define::cashmere::{EditOperationTypeEnum, EntityFieldEdit};
use manage_define::general_field_ids::ID_FIELD_ID;

use crate::utils::add_modify_update_fields;

pub async fn update_multi_entity_fields(
    edits: &Vec<EntityFieldEdit>,
    account_id: &str,
) -> Result<OperationResult, OperationResult> {
    let client = database::get_mongodb_client().await;
    let mut session = if let Ok(s) = client.start_session(None).await {
        s
    } else {
        return Err(operation_failed(
            "update_multi_entity_fields",
            t!("发起会话失败").to_string(),
        ));
    };

    let options = TransactionOptions::builder()
        .read_concern(ReadConcern::majority())
        .write_concern(WriteConcern::builder().w(Acknowledgment::Majority).build())
        .build();

    if let Err(err) = session.start_transaction(options).await {
        return Err(operation_failed(
            "update_multi_entity_fields",
            format!("{}: {}", t!("发起事务失败"), err),
        ));
    };

    // TODO: 事务提交失败后，重试提交事务
    match execute_transaction(edits, &mut session, account_id).await {
        Ok(_) => {
            if let Err(err) = session.commit_transaction().await {
                log::error!("{}: {}", t!("提交事务失败"), err);
                return Err(operation_failed(
                    "update_multi_entity_fields",
                    format!("{}: {:?}", t!("提交失败"), err),
                ));
            };
            Ok(operation_succeed("ok"))
        }
        Err(err) => {
            log::error!("{}: {}", t!("执行事务失败"), err);

            if let Err(err) = session.abort_transaction().await{
                log::error!("{}: {}", t!("回滚事务失败"), err);
            };

            Err(operation_failed(
                "update_multi_entity_fields",
                format!("{}: {:?}", t!("提交失败"), err),
            ))
        }
    }
}

async fn execute_transaction(
    // coll: &Collection<Document>,
    edits: &Vec<EntityFieldEdit>,
    session: &mut ClientSession,
    account_id: &str,
) -> DbResult<()> {
    log::debug!("{}: {}", t!("执行事务, 提交编辑数"), edits.len());
    // let client = session.client();
    // let database_name = get_database_name();
    // let database = client.database(database_name.as_str());
    let database = get_database().await;

    for edit in edits {
        let collection: Collection<Document> =
            database.collection(edit.manage_id.to_string().as_str());
        let query_doc = doc! {ID_FIELD_ID.to_string(): edit.entity_id.clone()};

        // 编辑单个字段
        match EditOperationTypeEnum::try_from(edit.operation_type).unwrap() {
            EditOperationTypeEnum::EditPrimaryField => {
                log::debug!(
                    "{}: {}-{}",
                    t!("编辑基础数据类型字段"),
                    edit.manage_id,
                    edit.field_id
                );

                let new_value_doc: Document = from_slice(&edit.edit).unwrap();
                let mut modify_doc = doc! {"$set": new_value_doc};
                let modify_doc = add_modify_update_fields(account_id, &mut modify_doc);
                match collection
                    .update_one_with_session(query_doc, modify_doc, None, session)
                    .await
                {
                    Ok(_) => continue,
                    Err(err) => {
                        log::error!(
                            "{}: {}-{}",
                            t!("编辑基础数据类型字段，更新到会话失败"),
                            edit.manage_id,
                            err
                        );
                        return Err(err);
                    }
                };
            }
            // 接收一个数组，添加元素到数组
            EditOperationTypeEnum::EditAddToArrayField => {
                log::debug!(
                    "{}: {}-{}",
                    t!("编辑数组字段，添加元素到数组"),
                    edit.manage_id,
                    edit.field_id
                );
                let new_value_doc: Document = from_slice(&edit.edit).unwrap();
                let items = new_value_doc
                    .get_array(edit.field_id.clone())
                    .unwrap()
                    .clone();
                let modify_doc = doc! {edit.field_id.clone():  doc! {"$each": items}};
                let mut modify_doc = doc! {"$addToSet": modify_doc};
                let modify_doc = add_modify_update_fields(account_id, &mut modify_doc);
                match collection
                    .update_one_with_session(query_doc, modify_doc, None, session)
                    .await
                {
                    Ok(_) => continue,
                    Err(err) => {
                        log::error!(
                            "{}: {}-{}",
                            t!("编辑数组字段，添加元素到数组，更新到会话失败"),
                            edit.manage_id,
                            err
                        );
                        return Err(err);
                    }
                };
            }
            // zh: 更新数组中的元素，根据索引值更新元素的值。
            EditOperationTypeEnum::EditUpdateArrayElementField => {
                log::debug!(
                    "{}: {}-{}",
                    t!("编辑数组字段，更新元素的字段值"),
                    edit.manage_id,
                    edit.field_id
                );
                let new_value_doc: Document = from_slice(&edit.edit).unwrap();
                let index = match new_value_doc.get_i32("index") {
                    Ok(i) => i,
                    Err(_err) => {
                        log::error!(
                            "{}: {}-{}",
                            t!("编辑数组字段，更新元素的字段值，索引值不存在"),
                            edit.manage_id,
                            edit.field_id
                        );
                        return Err(Error::custom("index not found"));
                    }
                };

                let f = format!("{}.{}", edit.field_id, index);
                let mut modify_doc = doc! {f: new_value_doc.get("index").unwrap().clone()};
                let modify_doc = add_modify_update_fields(account_id, &mut modify_doc);
                match collection
                    .update_one_with_session(
                        query_doc,
                        modify_doc,
                        UpdateOptions::builder().upsert(true).build(),
                        session,
                    )
                    .await
                {
                    Ok(_) => continue,
                    Err(err) => {
                        log::error!(
                            "{}: {}-{}",
                            t!("编辑数组字段，更新元素的字段值，更新到会话失败"),
                            edit.manage_id,
                            err
                        );
                        return Err(err);
                    }
                };
            }
            // zh: 接收一个数组，将数组中的元素从数组中移除。
            EditOperationTypeEnum::EditRemoveFromArrayField => {
                log::debug!(
                    "{}: {}-{}",
                    t!("编辑数组字段，从数组移除元素"),
                    edit.manage_id,
                    edit.field_id
                );
                let new_value_doc: Document = from_slice(&edit.edit).unwrap();
                let items: Vec<Bson> = new_value_doc
                    .get_array(edit.field_id.clone())
                    .unwrap()
                    .clone();

                let modify_doc = doc! {edit.field_id.clone(): doc! {"$in":items.clone()}};
                let mut modify_doc = doc! {"$pull": modify_doc};
                let modify_doc = add_modify_update_fields(account_id, &mut modify_doc);
                match collection
                    .update_one_with_session(query_doc.clone(), modify_doc, None, session)
                    .await
                {
                    Ok(_) => continue,
                    Err(err) => {
                        log::error!(
                            "{}: {}, {}",
                            t!("编辑数组字段，从数组移除元素，更新到会话失败"),
                            edit.manage_id,
                            err
                        );
                        return Err(err);
                    }
                };
            }
            // zh: 修改映射字段的值，单个字段
            EditOperationTypeEnum::EidtMapField => {
                log::debug!(
                    "{}: {}-{}",
                    t!("编辑映射字段"),
                    edit.manage_id,
                    edit.field_id
                );
                let new_value_doc: Document = from_slice(&edit.edit).unwrap();
                let item: Document = new_value_doc
                    .get_document(edit.field_id.clone())
                    .unwrap()
                    .clone();

                let key = item.keys().next().unwrap();
                let f = format!("{}.{}", edit.field_id, key);
                // log::debug!("{}: {}", t!("新值目标为"), f);

                let modify_doc = doc! {f:  item.get(key).unwrap()};
                let mut modify_doc = doc! {"$set": modify_doc};
                let modify_doc = add_modify_update_fields(account_id, &mut modify_doc);

                match collection
                    .update_one_with_session(
                        query_doc.clone(),
                        modify_doc,
                        UpdateOptions::builder().upsert(true).build(),
                        session,
                    )
                    .await
                {
                    Ok(_) => continue,
                    Err(err) => {
                        log::error!(
                            "{}: {}, {}",
                            t!("编辑映射字段, 添加更新到会话失败"),
                            edit.manage_id,
                            err
                        );
                        return Err(err);
                    }
                };
            }
            // zh: 移除映射字段
            EditOperationTypeEnum::EditMapFieldRemoveKey => {
                log::debug!(
                    "{}: {}-{}",
                    t!("编辑映射字段, 移除key"),
                    edit.manage_id,
                    edit.field_id
                );
                let new_value_doc: Document = from_slice(&edit.edit).unwrap();
                let key = new_value_doc.get_str("key").unwrap().to_string();
                let f = format!("{}.{}", edit.field_id, key);
                let mut modify_doc = doc! {f: bson::Bson::Null};

                let modify_doc = add_modify_update_fields(account_id, &mut modify_doc);
                match collection
                    .update_one_with_session(
                        query_doc,
                        modify_doc,
                        UpdateOptions::builder().upsert(true).build(),
                        session,
                    )
                    .await
                {
                    Ok(_) => continue,
                    Err(err) => {
                        log::error!(
                            "{}: {}, {}",
                            t!("编辑映射字段, 移除key, 添加更新到会话失败"),
                            edit.manage_id,
                            err
                        );
                        return Err(err);
                    }
                };
            }
        }
    }
    Ok(())
}
