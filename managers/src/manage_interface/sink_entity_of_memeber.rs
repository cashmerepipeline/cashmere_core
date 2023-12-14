use cash_result::{add_call_name_to_chain, operation_failed, OperationResult};
use database::get_database;
use dependencies_sync::{
    bson::{doc, Document},
    log,
    mongodb::{
        options::{Acknowledgment, ReadConcern, TransactionOptions, WriteConcern},
        Collection,
    },
    rust_i18n::{self, t},
};
use manage_define::{
    field_ids::{
        MEMBERS_OWNER_ENTITY_ID_FIELD_ID, MEMBERS_OWNER_MANAGE_ID_FIELD_ID,
        MEMBERS_SELF_ENTITY_ID_FIELD_ID, MEMBERS_SELF_MANAGE_ID_FIELD_ID,
    },
    general_field_ids::*,
    manage_ids::MEMBERS_MANAGE_ID,
};

use crate::entity_cache_map::cache_update_entity;

pub async fn sink_entity_of_member(
    owner_manage_id: &str,
    owner_entity_id: &str,
    self_manage_id: &str,
    new_entity_doc: &mut Document,
    account_id: &str,
    group_id: &str,
) -> Result<String, OperationResult> {
    let client = database::get_mongodb_client().await;
    let database = get_database().await;

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

    let member_collection: Collection<Document> =
        database.collection(MEMBERS_MANAGE_ID.to_string().as_str());
    let entity_collection: Collection<Document> =
        database.collection(self_manage_id.to_string().as_str());

    let new_entity_id = new_entity_doc
        .get_str(ID_FIELD_ID.to_string())
        .unwrap()
        .to_owned();
    // 创建标记
    new_entity_doc.insert(CREATOR_FIELD_ID.to_string(), account_id);
    new_entity_doc.insert(MODIFIER_FIELD_ID.to_string(), account_id);
    new_entity_doc.insert(OWNER_FIELD_ID.to_string(), account_id);
    new_entity_doc.insert(GROUPS_FIELD_ID.to_string(), vec![group_id]);

    let mut new_member_doc = Document::new();
    new_member_doc.insert(
        MEMBERS_OWNER_MANAGE_ID_FIELD_ID.to_string(),
        owner_manage_id,
    );
    new_member_doc.insert(
        MEMBERS_OWNER_ENTITY_ID_FIELD_ID.to_string(),
        owner_entity_id,
    );
    new_member_doc.insert(MEMBERS_SELF_MANAGE_ID_FIELD_ID.to_string(), self_manage_id);
    new_member_doc.insert(
        MEMBERS_SELF_ENTITY_ID_FIELD_ID.to_string(),
        new_entity_id.clone(),
    );

    if let Err(err) = entity_collection
        .insert_one_with_session(new_entity_doc, None, &mut session)
        .await
    {
        log::error!("{}: {}-{}", t!("插入新实体到会话失败"), self_manage_id, err);

        return Err(operation_failed(
            "sink_entity_of_member",
            format!("{}: {}", t!("插入新实体到会话失败"), err),
        ));
    };

    if let Err(err) = member_collection
        .insert_one_with_session(new_member_doc.clone(), None, &mut session)
        .await
    {
        log::error!("{}: {}-{}", t!("插入新成员到会话失败"), self_manage_id, err);

        return Err(operation_failed(
            "sink_entity_of_member",
            format!("{}: {}", t!("插入新成员到会话失败"), err),
        ));
    };

    let query_doc = doc! {
        ID_FIELD_ID.to_string(): new_entity_id.clone(),
    };
    let update_timestamp_doc = doc! {
      "$currentDate": doc!{
              MODIFY_TIMESTAMP_FIELD_ID.to_string(): {"$type":"timestamp"},
              CREATE_TIMESTAMP_FIELD_ID.to_string(): {"$type":"timestamp"},
      }
    };

    if let Err(err) = entity_collection
        .update_one_with_session(query_doc, update_timestamp_doc.clone(), None, &mut session)
        .await
    {
        log::error!("{}: {}-{}", t!("更新实体到会话失败"), self_manage_id, err);

        return Err(operation_failed(
            "sink_entity_of_member",
            format!("{}: {}", t!("更新实体时间戳到会话失败"), err),
        ));
    };
    if let Err(err) = member_collection
        .update_one_with_session(new_member_doc, update_timestamp_doc, None, &mut session)
        .await
    {
        log::error!("{}: {}-{}", t!("更新成员到会话失败"), self_manage_id, err);

        return Err(operation_failed(
            "sink_entity_of_member",
            format!("{}: {}", t!("更新成员时间戳到会话失败"), err),
        ));
    };

    if let Err(err) = session.commit_transaction().await {
        log::error!("{}: {}", t!("执行事务失败"), err);
        session.abort_transaction().await;
        return Err(operation_failed(
            "sink_entity_of_member",
            format!("{}: {:?}", t!("提交事务失败"), err),
        ));
    };

    Ok(new_entity_id)
}
