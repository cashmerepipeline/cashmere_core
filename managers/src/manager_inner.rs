use dependencies_sync::tonic::async_trait;
use cash_core::PropertyField;
use std::sync::Arc;
use cash_core::Manage;
use cash_result::OperationResult;
use dependencies_sync::bson::Document;
use dependencies_sync::parking_lot::RwLock;
use crate::ManagerTrait;

#[derive(Clone)]
pub struct ManagerInner {
    pub manager: Arc<dyn ManagerTrait>,
}

#[async_trait]
impl ManagerTrait for ManagerInner {
    async fn get_manage_schema(&self) -> Vec<PropertyField> {
        self.manager.get_manage_schema().await
    }

    async fn get_manage_document(&self) -> Arc<RwLock<Document>> {
        self.manager.get_manage_document().await
    }

    async fn get_schema_document(&self) -> Result<Document, OperationResult> {
        self.manager.get_schema_document().await
    }

    async fn sink_entity(
        &self,
        new_entity_doc: &mut Document,
        account_id: &str,
        group_id: &str,
    ) -> Result<String, OperationResult> {
        self.manager
            .sink_entity(new_entity_doc, account_id, group_id)
            .await
    }

    async fn get_entity_by_id(&self, entity_id: &String) -> Result<Document, OperationResult> {
        self.manager.get_entity_by_id(entity_id).await
    }

    async fn get_entities_by_filter(
        &self,
        filter: &Option<Document>,
    ) -> Result<Vec<Document>, OperationResult> {
        self.manager.get_entities_by_filter(filter).await
    }

    async fn mark_entity_removed(
        &self,
        entity_id: &String,
        account_id: &str,
    ) -> Result<OperationResult, OperationResult> {
        self.manager
            .mark_entity_removed(entity_id, account_id)
            .await
    }

    async fn init(&self) -> Result<OperationResult, OperationResult> {
        self.manager.init().await
    }

    fn unregister(&self) -> Result<OperationResult, OperationResult> {
        self.manager.unregister()
    }

    fn get_id(&self) -> i32 {
        self.manager.get_id()
    }

    fn get_name(&self) -> String {
        self.manager.get_name()
    }

    fn has_cache(&self) -> bool {
        self.manager.has_cache()
    }

    async fn get_manage(&self) -> Arc<RwLock<Manage>> {
        self.manager.get_manage().await
    }

    async fn get_new_entity_id(&self, account_id: &str) -> Option<i64> {
        self.manager.get_new_entity_id(account_id).await
    }
}
