use cash_result::{operation_failed, operation_succeed, OperationResult};
use dependencies_sync::bson::{self, doc, from_bson, from_slice, Document};
use dependencies_sync::mongodb::error::{
    Result as DbResult, TRANSIENT_TRANSACTION_ERROR, UNKNOWN_TRANSACTION_COMMIT_RESULT,
};
use dependencies_sync::mongodb::options::{
    Acknowledgment, ReadConcern, TransactionOptions, UpdateOptions, WriteConcern,
};
use dependencies_sync::mongodb::ClientSession;

use dependencies_sync::rust_i18n::{self, t};
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
            format!("{}", t!("发起会话失败")),
        ));
    };

    let options = TransactionOptions::builder()
        .read_concern(ReadConcern::majority())
        .write_concern(WriteConcern::builder().w(Acknowledgment::Majority).build())
        .build();

    if let Err(err) = session.start_transaction(options).await {
        return Err(operation_failed(
            "update_multi_entity_fields",
            format!("{}: {}", t!(""), err),
        ));
    };

    // A "TransientTransactionError" label indicates that the entire transaction can be retried
    // with a reasonable expectation that it will succeed.
    while let Err(error) = execute_transaction(&edits, &mut session, account_id).await {
        if !error.contains_label(TRANSIENT_TRANSACTION_ERROR) {
            break;
        }
    }

    Ok(operation_succeed("ok"))
}

async fn execute_transaction(
    // coll: &Collection<Document>,
    edits: &Vec<EntityFieldEdit>,
    session: &mut ClientSession,
    account_id: &str,
) -> DbResult<()> {
    for edit in edits {
        let collection = database::get_collection_by_id(edit.manage_id.to_string().as_str())
            .await
            .unwrap();

        let query_doc = doc! {ID_FIELD_ID.to_string(): edit.entity_id.clone()};
        match EditOperationTypeEnum::try_from(edit.operation_type).unwrap() {
            EditOperationTypeEnum::EditPrimaryField => {
                let new_value_doc: Document = from_slice(&edit.edit).unwrap();
                let mut modify_doc = doc! {"$set": new_value_doc};
                let modify_doc = add_modify_update_fields(account_id, &mut modify_doc);
                collection
                    .update_one_with_session(query_doc, modify_doc, None, session)
                    .await?;
            }
            EditOperationTypeEnum::EditAddToArrayField => {
                let new_value_doc: Document = from_slice(&edit.edit).unwrap();
                let items = new_value_doc
                    .get_array(edit.field_id.clone())
                    .unwrap()
                    .clone();
                let modify_doc = doc! {edit.field_id.clone():  doc! {"$each": items}};
                let mut modify_doc = doc! {"$addToSet": modify_doc};
                let modify_doc = add_modify_update_fields(account_id, &mut modify_doc);
                collection.update_one_with_session(query_doc, modify_doc, None, session).await;
            }
            EditOperationTypeEnum::EditRemoveFromArrayField => {
                let new_value_doc: Document = from_slice(&edit.edit).unwrap();
                let items: Vec<Document> = new_value_doc
                    .get_array(edit.field_id.clone())
                    .unwrap()
                    .iter()
                    .map(|x| from_bson::<Document>(x.clone()).unwrap().clone())
                    .collect();
                for item in items {
                    let key = item.get_str("key").unwrap();
                    let modify_doc = doc! {edit.field_id.clone(): item.get(key).unwrap()};
                    let mut modify_doc = doc! {"$pull": modify_doc};
                    let modify_doc = add_modify_update_fields(account_id, &mut modify_doc);
                    collection.update_one_with_session(
                        query_doc.clone(),
                        modify_doc,
                        None,
                        session,
                    ).await;
                }
            }
            EditOperationTypeEnum::EidtMapField => {
                let new_value_doc: Document = from_slice(&edit.edit).unwrap();
                let items: Vec<Document> = new_value_doc
                    .get_array(edit.field_id.clone())
                    .unwrap()
                    .iter()
                    .map(|x| from_bson::<Document>(x.clone()).unwrap().clone())
                    .collect();

                for item in items {
                    let key = item.keys().next().unwrap();
                    let f = format!("{}.{}", edit.field_id, key);
                    let modify_doc = doc! {f:  new_value_doc.get(key).unwrap()};
                    let mut modify_doc = doc! {"$set": modify_doc};
                    let modify_doc = add_modify_update_fields(account_id, &mut modify_doc);
                    collection.update_one_with_session(
                        query_doc.clone(),
                        modify_doc,
                        UpdateOptions::builder().upsert(true).build(),
                        session,
                    ).await;
                }
            }
            EditOperationTypeEnum::EditMapFieldRemoveKey => {
                let new_value_doc: Document = from_slice(&edit.edit).unwrap();
                let key = new_value_doc.get_str("key").unwrap().to_string();
                let f = format!("{}.{}", edit.field_id, key);
                let mut modify_doc = doc! {f: bson::Bson::Null};

                let modify_doc = add_modify_update_fields(account_id, &mut modify_doc);
                collection.update_one_with_session(
                    query_doc,
                    modify_doc,
                    UpdateOptions::builder().upsert(true).build(),
                    session,
                ).await;
            }
            EditOperationTypeEnum::EditUpdateArrayElementField => {
                let new_value_doc: Document = from_slice(&edit.edit).unwrap();
                let index = new_value_doc.get_i32("index").unwrap();
                let f = format!("{}.{}", edit.field_id, index);

                let mut modify_doc = doc! {f: new_value_doc.get("index").unwrap().clone()};
                let modify_doc = add_modify_update_fields(account_id, &mut modify_doc);
                collection.update_one_with_session(
                    query_doc,
                    modify_doc,
                    UpdateOptions::builder().upsert(true).build(),
                    session,
                ).await;
            }
        }
    }

    // An "UnknownTransactionCommitResult" label indicates that it is unknown whether the
    // commit has satisfied the write concern associated with the transaction. If an error
    // with this label is returned, it is safe to retry the commit until the write concern is
    // satisfied or an error without the label is returned.
    loop {
        let result = session.commit_transaction().await;
        if let Err(ref error) = result {
            if error.contains_label(UNKNOWN_TRANSACTION_COMMIT_RESULT) {
                continue;
            }
        }
        result?
    }
}
