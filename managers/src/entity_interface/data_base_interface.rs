use cash_result::{add_call_name_to_chain, OperationResult};
use dependencies_sync::{
    bson::{doc, Document},
    tokio_stream::wrappers::ReceiverStream, tonic::async_trait,
};
use entity::hard_code_cache::{get_hard_coded_cache_map};
use manage_define::general_field_ids::*;

use crate::{hard_coded_cache_interface::HardCodedInterface, ManagerInterface};

use super::{get_entity_by_id, get_entity_stream};

#[async_trait]
pub trait EntityInterface
where
    Self: ManagerInterface + HardCodedInterface, 
{
    // ---------------------
    // 实体相关操作
    // ---------------------

    /// 实体是否可以删除，默认不可删除，使用removed字段
    fn is_entity_deleteable(&self) -> bool {
        false
    }

    async fn count_entity(&self, filter_doc: Document) -> Result<u64, OperationResult> {
        let manage_id = self.get_id();
        entity::count_entity(manage_id, filter_doc).await
    }

    async fn get_entry_counts(&self) -> Result<u64, OperationResult> {
        let manage_id = self.get_id();

        entity::get_entry_count(manage_id).await
    }

    /// 取得新实体id, 针对数量有限相对固定的管理使用, 不需要使用id的情况需要重写本方法
    async fn get_new_entity_id(&self, account_id: &str) -> Option<i64> {
        let manage_id = self.get_id().to_string();
        entity::get_new_entity_id(&manage_id.to_string(), account_id).await
    }

    // 新建实体
    async fn sink_entity(
        &self,
        new_entity_doc: &mut Document,
        account_id: &str,
        group_id: &str,
    ) -> Result<String, OperationResult> {
        super::sink_entity(self.get_id(), new_entity_doc, account_id, group_id).await
    }

    /// 通过id取得实体
    async fn get_entity_by_id(
        &self,
        entity_id: &str,
        present_fields: &[String],
        no_present_fields: &[String],
    ) -> Result<Document, OperationResult> {
        let manage_id = self.get_id();
        let is_hard_coded = self.is_hard_coded().await;

        get_entity_by_id(
            manage_id,
            entity_id,
            is_hard_coded,
            present_fields,
            no_present_fields,
        )
        .await
    }

    /// 通过过滤取得实体
    async fn get_entities_by_filter(
        &self,
        filter: &Option<Document>,
    ) -> Result<Vec<Document>, OperationResult> {
        let manage_id = self.get_id();

        if HardCodedInterface::is_hard_coded(self).await {
            let entities = {
                let c_map = get_hard_coded_cache_map(manage_id).await.unwrap();
                let e_map = c_map.read();
                e_map.values().cloned().collect::<Vec<Document>>()
            };
            return Ok(entities);
        }

        // zh: 从数据库中取得实体
        match entity::get_entities(manage_id, filter, &vec![]).await {
            Ok(r) => Ok(r),
            Err(e) => Err(add_call_name_to_chain(e, "get_entity_by_id".to_string())),
        }
    }

    /// 取得条件排序分页
    async fn get_entities_by_page(
        &self,
        page_index: u32,
        matches: &Option<Document>,
        sorts: &Option<Document>,
        unsets: &Vec<String>,
    ) -> Result<Vec<Document>, OperationResult> {
        let manage_id = self.get_id().to_string();

        match entity::get_entities_by_page(&manage_id, page_index, matches, sorts, unsets).await {
            Ok(r) => Ok(r),
            Err(e) => Err(add_call_name_to_chain(
                e,
                "manager::get_entities_by_page".to_string(),
            )),
        }
    }

    async fn get_entity_stream(
        &self,
        matche_doc: Document,
        unsets: &[String],
        sorts: Option<Document>,
        start_oid: Option<&str>,
        skip_count: u32,
    ) -> Result<ReceiverStream<Document>, OperationResult> {
        let manage_id = self.get_id();
        let hard_coded = self.is_hard_coded().await;
        get_entity_stream(
            manage_id, matche_doc, unsets, sorts, start_oid, skip_count, hard_coded,
        )
        .await
    }

    async fn update_entity_field(
        &self,
        query_doc: Document,
        modify_doc: &mut Document,
        account_id: &str,
    ) -> Result<OperationResult, OperationResult> {
        let manage_id = self.get_id().to_string();
        match entity::update_entity_field(&manage_id.to_string(), query_doc, modify_doc, account_id)
            .await
        {
            Ok(r) => Ok(r),
            Err(e) => Err(add_call_name_to_chain(
                e,
                "manager::update_entity_field".to_string(),
            )),
        }
    }

    // 标记为移除
    async fn mark_entity_removed(
        &self,
        entity_id: &String,
        account_id: &str,
    ) -> Result<OperationResult, OperationResult> {
        let manage_id = self.get_id();
        let q_doc = doc! {
            ID_FIELD_ID.to_string(): entity_id
        };
        let mut m_doc = doc! {
            REMOVED_FIELD_ID.to_string(): true
        };
        match entity::update_entity_field(manage_id, q_doc, &mut m_doc, account_id).await {
            Ok(r) => Ok(r),
            Err(e) => Err(add_call_name_to_chain(e, "mark_entity_removed".to_string())),
        }
    }

    async fn recover_removed_entity(
        &self,
        entity_id: &String,
        account_id: &str,
    ) -> Result<OperationResult, OperationResult> {
        let manage_id = self.get_id();
        let q_doc = doc! {
            ID_FIELD_ID.to_string(): entity_id
        };
        let mut m_doc = doc! {
            REMOVED_FIELD_ID.to_string(): false
        };
        match entity::update_entity_field(manage_id, q_doc, &mut m_doc, account_id).await {
            Ok(r) => Ok(r),
            Err(e) => Err(add_call_name_to_chain(e, "mark_entity_removed".to_string())),
        }
    }

    async fn is_mark_removed(&self, entity_id: &str) -> bool {
        let manage_id = self.get_id();
        match entity::get_entity_by_id(manage_id, entity_id, &[], &[]).await {
            Ok(r) => {
                if let Ok(b) = r.get_bool(REMOVED_FIELD_ID.to_string()) {
                    return b;
                } else {
                    false
                }
            }
            Err(_e) => false,
        }
    }

    async fn entity_exists(&self, query_doc: &Document) -> Option<String> {
        let manage_id = self.get_id();
        entity::entity_exists(manage_id, query_doc).await
    }

    async fn delete_entity(
        &self,
        query_doc: &Document,
    ) -> Result<OperationResult, OperationResult> {
        let manage_id = self.get_id();
        match entity::delete_entity(manage_id, query_doc).await {
            Ok(r) => Ok(r),
            Err(e) => Err(add_call_name_to_chain(e, "delete_entity".to_string())),
        }
    }

    //-------------------------
    //  数组属性操作
    /// 添加数组元素, 不重复
    async fn add_to_array_field(
        &self,
        query_doc: Document,
        modify_doc: Document,
        account_id: &str,
    ) -> Result<OperationResult, OperationResult> {
        let manage_id = self.get_id().to_string();
        match entity::add_to_array_field(&manage_id.to_string(), query_doc, modify_doc, account_id)
            .await
        {
            Ok(r) => Ok(r),
            Err(e) => Err(add_call_name_to_chain(e, "add_to_array_field".to_string())),
        }
    }

    /// 移除数组元素
    async fn remove_from_array_field(
        &self,
        query_doc: Document,
        modify_doc: Document,
        account_id: &str,
    ) -> Result<OperationResult, OperationResult> {
        let manage_id = self.get_id().to_string();
        match entity::remove_from_array_field(
            &manage_id.to_string(),
            query_doc,
            modify_doc,
            account_id,
        )
        .await
        {
            Ok(r) => Ok(r),
            Err(e) => Err(add_call_name_to_chain(
                e,
                "manager::remove_from_array_field".to_string(),
            )),
        }
    }

    /// 更新数组元素属性
    async fn update_array_element_field(
        &self,
        query_doc: Document,
        modify_doc: Document,
        account_id: &str,
    ) -> Result<OperationResult, OperationResult> {
        let manage_id = self.get_id().to_string();
        match entity::update_entity_array_element_field(
            &manage_id.to_string(),
            query_doc,
            modify_doc,
            account_id,
        )
        .await
        {
            Ok(r) => Ok(r),
            Err(e) => Err(add_call_name_to_chain(
                e,
                "update_array_element_field".to_string(),
            )),
        }
    }

    //-------------------------
    //  映射属性操作
    /// 添加映射字段
    async fn insert_entity_map_field(
        &self,
        query_doc: Document,
        modify_doc: Document,
        account_id: &str,
    ) -> Result<OperationResult, OperationResult> {
        let manage_id = self.get_id().to_string();
        match entity::insert_entity_map_field(&manage_id, query_doc, modify_doc, account_id).await {
            Ok(r) => Ok(r),
            Err(e) => Err(add_call_name_to_chain(
                e,
                "insert_entity_map_field".to_string(),
            )),
        }
    }

    async fn query_entity_map_field(
        &self,
        query_doc: &Document,
        account_id: &str,
    ) -> Result<Document, OperationResult> {
        let manage_id = self.get_id().to_string();
        match entity::query_entity_map_field(&manage_id, query_doc, account_id).await {
            Ok(r) => Ok(r),
            Err(e) => Err(add_call_name_to_chain(
                e,
                "query_entity_map_field".to_string(),
            )),
        }
    }

    /// 更新映射字段
    async fn update_entity_map_field(
        &self,
        query_doc: Document,
        modify_doc: Document,
        account_id: &str,
    ) -> Result<OperationResult, OperationResult> {
        let manage_id = self.get_id().to_string();
        match entity::update_entity_map_field(&manage_id, query_doc, modify_doc, account_id).await {
            Ok(r) => Ok(r),
            Err(e) => Err(add_call_name_to_chain(
                e,
                "update_entity_map_field".to_string(),
            )),
        }
    }
    /// 删除映射字段
    async fn delete_entity_map_field_key(
        &self,
        query_doc: Document,
        modify_doc: Document,
        account_id: &str,
    ) -> Result<OperationResult, OperationResult> {
        let manage_id = self.get_id().to_string();
        match entity::delete_entity_map_field_key(&manage_id, query_doc, modify_doc, account_id)
            .await
        {
            Ok(r) => Ok(r),
            Err(e) => Err(add_call_name_to_chain(
                e,
                "update_entity_map_field".to_string(),
            )),
        }
    }
}
