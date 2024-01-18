use crate::manager_inner::ManagerInner;
use crate::ManagerTrait;
use cash_core::Manage;
use cash_core::SchemaField;
use cash_result::OperationResult;
use dependencies_sync::bson::Document;
use dependencies_sync::parking_lot::RwLock;
use dependencies_sync::tonic::async_trait;
use manage_define::cashmere::EntityFieldEdit;
use std::sync::Arc;

/// 管理器包裹
#[derive(Clone)]
pub struct Manager {
    pub inner: Arc<ManagerInner>,
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
impl ManagerTrait for Manager {
    async fn get_manage_schema(&self) -> Vec<SchemaField> {
        self.inner.get_manage_schema().await
    }

    async fn get_manage_document(&self) -> Arc<RwLock<Document>> {
        self.inner.get_manage_document().await
    }

    async fn get_schema_document(&self) -> Result<Document, OperationResult> {
        self.inner.get_schema_document().await
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

    async fn entity_exists(&self, query_doc: &Document) -> Option<String> {
        self.inner.entity_exists(query_doc).await
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

    fn has_cache(&self) -> bool {
        self.inner.has_cache()
    }

    async fn get_manage(&self) -> Arc<RwLock<Manage>> {
        self.inner.get_manage().await
    }

    async fn get_new_entity_id(&self, account_id: &str) -> Option<i64> {
        self.inner.get_new_entity_id(account_id).await
    }
}
