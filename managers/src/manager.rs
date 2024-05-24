use std::sync::Arc;

use cash_core::Manage;
use cash_core::SchemaField;
use cash_result::OperationResult;
use dependencies_sync::bson::Document;
use dependencies_sync::parking_lot::RwLock;
use dependencies_sync::tokio_stream::wrappers::ReceiverStream;
use dependencies_sync::tonic::async_trait;
use manage_define::cashmere::EntityFieldEdit;

use crate::entity_interface::EntityInterface;
use crate::hard_coded_cache_interface::HardCodedInterface;
use crate::ManagerInterface;

#[async_trait]
pub trait AllManagerInterface: ManagerInterface + EntityInterface + HardCodedInterface {}

/// 管理器包裹
#[derive(Clone)]
pub struct Manager {
    pub inner: Arc<Box<dyn 'static + AllManagerInterface>>,
}

/// 不和具体管理绑定的方法
impl Manager {
    pub async fn update_multi_entity_fields(
        edits: &Vec<EntityFieldEdit>,
        account_id: &str,
    ) -> Result<OperationResult, OperationResult> {
        entity::update_multi_entity_fields(edits, account_id).await
    }
}

#[async_trait]
impl AllManagerInterface for Manager {}

#[async_trait]
impl HardCodedInterface for Manager {
    async fn is_hard_coded(&self) -> bool {
        self.inner.is_hard_coded().await
    }
}

#[async_trait]
impl ManagerInterface for Manager {
    async fn get_manage_schema(&self) -> Vec<SchemaField> {
        self.inner.get_manage_schema().await
    }

    async fn get_manage_document(&self) -> Arc<RwLock<Document>> {
        self.inner.get_manage_document().await
    }

    async fn get_schema_document(&self) -> Result<Document, OperationResult> {
        self.inner.get_schema_document().await
    }

    async fn init_check(&self) -> Result<OperationResult, OperationResult> {
        self.inner.init_check().await
    }

    fn unregister(&self) -> Result<OperationResult, OperationResult> {
        self.inner.unregister()
    }

    fn get_id(&self) -> &'static str {
        self.inner.get_id()
    }

    fn get_name(&self) -> String {
        self.inner.get_name()
    }

    async fn get_manage(&self) -> Arc<RwLock<Manage>> {
        self.inner.get_manage().await
    }

    // async fn get_new_entity_id(&self, account_id: &str) -> Option<i64> {
    //     self.inner.get_new_entity_id(account_id).await
    // }
}

#[async_trait]
impl EntityInterface for Manager {
    fn is_entity_deleteable(&self) -> bool {
        self.inner.is_entity_deleteable()
    }

    async fn count_entity(&self, filter_doc: Document) -> Result<u64, OperationResult> {
        self.inner.count_entity(filter_doc).await
    }

    async fn get_entry_counts(&self) -> Result<u64, OperationResult> {
        self.inner.get_entry_counts().await
    }

    async fn get_new_entity_id(&self, account_id: &str) -> Option<i64> {
        self.inner.get_new_entity_id(account_id).await
    }

    async fn sink_entity(
        &self,
        new_entity_doc: &mut Document,
        account_id: &str,
        group_id: &str,
    ) -> Result<String, OperationResult> {
        self.inner
            .sink_entity(new_entity_doc, account_id, group_id)
            .await
    }

    async fn get_entity_by_id(
        &self,
        entity_id: &str,
        present_fields: &[String],
        no_present_fields: &[String],
    ) -> Result<Document, OperationResult> {
        self.inner
            .get_entity_by_id(entity_id, present_fields, no_present_fields)
            .await
    }

    async fn get_entities_by_filter(
        &self,
        filter: &Option<Document>,
    ) -> Result<Vec<Document>, OperationResult> {
        self.inner.get_entities_by_filter(filter).await
    }

    async fn get_entities_by_page(
        &self,
        page_index: u32,
        matches: &Option<Document>,
        sorts: &Option<Document>,
        unsets: &Vec<String>,
    ) -> Result<Vec<Document>, OperationResult> {
        self.inner
            .get_entities_by_page(page_index, matches, sorts, unsets)
            .await
    }

    async fn get_entity_stream(
        &self,
        matche_doc: Document,
        unsets: &[String],
        sorts: Option<Document>,
        start_oid: Option<&str>,
        skip_count: u32,
    ) -> Result<ReceiverStream<Document>, OperationResult> {
        self.inner
            .get_entity_stream(matche_doc, unsets, sorts, start_oid, skip_count)
            .await
    }

    async fn update_entity_field(
        &self,
        query_doc: Document,
        modify_doc: &mut Document,
        account_id: &str,
    ) -> Result<OperationResult, OperationResult> {
        self.inner
            .update_entity_field(query_doc, modify_doc, account_id)
            .await
    }

    async fn mark_entity_removed(
        &self,
        entity_id: &String,
        account_id: &str,
    ) -> Result<OperationResult, OperationResult> {
        self.inner.mark_entity_removed(entity_id, account_id).await
    }

    async fn recover_removed_entity(
        &self,
        entity_id: &String,
        account_id: &str,
    ) -> Result<OperationResult, OperationResult> {
        self.inner
            .recover_removed_entity(entity_id, account_id)
            .await
    }

    async fn is_mark_removed(&self, entity_id: &str) -> bool {
        self.inner.is_mark_removed(entity_id).await
    }

    async fn entity_exists(&self, query_doc: &Document) -> Option<String> {
        self.inner.entity_exists(query_doc).await
    }

    async fn delete_entity(
        &self,
        query_doc: &Document,
    ) -> Result<OperationResult, OperationResult> {
        self.inner.delete_entity(query_doc).await
    }

    async fn add_to_array_field(
        &self,
        query_doc: Document,
        modify_doc: Document,
        account_id: &str,
    ) -> Result<OperationResult, OperationResult> {
        self.inner
            .add_to_array_field(query_doc, modify_doc, account_id)
            .await
    }

    async fn remove_from_array_field(
        &self,
        query_doc: Document,
        modify_doc: Document,
        account_id: &str,
    ) -> Result<OperationResult, OperationResult> {
        self.inner
            .remove_from_array_field(query_doc, modify_doc, account_id)
            .await
    }

    async fn update_array_element_field(
        &self,
        query_doc: Document,
        modify_doc: Document,
        account_id: &str,
    ) -> Result<OperationResult, OperationResult> {
        self.inner
            .update_array_element_field(query_doc, modify_doc, account_id)
            .await
    }

    async fn insert_entity_map_field(
        &self,
        query_doc: Document,
        modify_doc: Document,
        account_id: &str,
    ) -> Result<OperationResult, OperationResult> {
        self.inner
            .insert_entity_map_field(query_doc, modify_doc, account_id)
            .await
    }

    async fn query_entity_map_field(
        &self,
        query_doc: &Document,
        account_id: &str,
    ) -> Result<Document, OperationResult> {
        self.inner
            .query_entity_map_field(query_doc, account_id)
            .await
    }

    async fn update_entity_map_field(
        &self,
        query_doc: Document,
        modify_doc: Document,
        account_id: &str,
    ) -> Result<OperationResult, OperationResult> {
        self.inner
            .update_entity_map_field(query_doc, modify_doc, account_id)
            .await
    }

    async fn delete_entity_map_field_key(
        &self,
        query_doc: Document,
        modify_doc: Document,
        account_id: &str,
    ) -> Result<OperationResult, OperationResult> {
        self.inner
            .delete_entity_map_field_key(query_doc, modify_doc, account_id)
            .await
    }
}
