/*
Author: 闫刚
lib.rs (c) 2020
Desc: 大管家
Created:  2020-11-16T11:08:01.884Z
Modified: !date!
*/

// 异步线程限制
#![recursion_limit = "256"]

use std::sync::Arc;

use async_trait::async_trait;
use bson::Document;
use parking_lot::RwLock;

use cash_core::Manage;
use cash_result::*;
use property_field::PropertyField;
use traits::ManagerTrait;

pub mod manages_manager;
pub mod accounts_manager;
pub mod groups_manager;
pub mod events_manager;
pub mod event_queues_manager;
pub mod event_handles_manager;
pub mod tasks_manager;
pub mod datas_manager;
pub mod areas_manager;
pub mod messages_manager;
pub mod message_handles_manager;
pub mod persons_manager;
pub mod view_rules_manager;
pub mod templates_manager;

pub mod orgnizations_manager;
pub mod departments_manager;
pub mod questions_manager;
pub mod comments_manager;
pub mod answers_manager;

pub mod graph_know;
pub mod imix;
pub mod knitter;

pub mod traits;
mod macros;
pub(crate) mod schema;
pub mod tag_types_manager;
pub mod show_settings_manager;

// use log::{info};

/// 管理器包装
#[derive(Clone)]
pub struct Manager {
    pub inner: Arc<ManagerInner>
}

#[derive(Clone)]
pub struct ManagerInner {
    pub manager: Arc<dyn ManagerTrait>
}

#[async_trait]
impl ManagerTrait for Manager {
    async fn get_manage_schema(&self) -> Option<Vec<PropertyField>> {
        self.inner.get_manage_schema().await
    }

    async fn get_manage_document(&self) -> Arc<RwLock<Document>> {
        self.inner.get_manage_document().await
    }

    async fn get_schema_document(&self) -> Result<bson::Bson, OperationResult> {
        self.inner.get_schema_document().await
    }

    async fn new_entity(
        &self,
        new_entity_doc: &mut Document,
        account_id: &String,
        group_id: &String,
    ) -> Result<OperationResult, OperationResult> {
        self.inner.new_entity(new_entity_doc, account_id, group_id).await
    }

    async fn entity_exists(
        &self,
        query_doc: Document
    ) -> bool {
        self.inner.entity_exists(query_doc).await
    }

    async fn get_entity_by_id(
        &self,
        entity_id: &String
    ) -> Result<Document, OperationResult> {
        self.inner.get_entity_by_id(entity_id).await
    }

    async fn get_entities_by_filter(
        &self,
        filter: &Option<Document>
    ) -> Result<Vec<Document>, OperationResult> {
        self.inner.get_entities_by_filter(filter).await
    }

    async fn mark_entity_removed(
        &self,
        entity_id: &String,
        account_id: &String,
    ) -> Result<OperationResult, OperationResult> {
        self.inner.mark_entity_removed(entity_id, account_id).await
    }

    async fn init(&self) -> Result<OperationResult, OperationResult> {
        self.inner.init().await
    }

    fn unregister(&self) -> Result<OperationResult, OperationResult> {
        self.inner.unregister()
    }

    fn get_manager_id(&self) -> i32 {
        self.inner.get_manager_id()
    }

    fn get_manager_name(&self) -> String {
        self.inner.get_manager_name()
    }

    fn has_cache(&self) -> bool {
        self.inner.has_cache()
    }

    async fn get_manage(&self) -> Arc<RwLock<Manage>> {
        self.inner.get_manage().await
    }

    async fn get_new_entity_id(&self) -> Option<i64> {
        self.inner.get_new_entity_id().await
    }
}


#[async_trait]
impl ManagerTrait for ManagerInner {
    async fn get_manage_schema(&self) -> Option<Vec<PropertyField>> {
        self.manager.get_manage_schema().await
    }

    async fn get_manage_document(&self) -> Arc<RwLock<Document>> {
        self.manager.get_manage_document().await
    }

    async fn get_schema_document(&self) -> Result<bson::Bson, OperationResult> {
        self.manager.get_schema_document().await
    }

    async fn new_entity(
        &self,
        new_entity_doc: &mut Document,
        account_id: &String,
        group_id: &String,
    ) -> Result<OperationResult, OperationResult> {
        self.manager.new_entity(new_entity_doc, account_id, group_id).await
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
        account_id: &String,
    ) -> Result<OperationResult, OperationResult> {
        self.manager.mark_entity_removed(entity_id, account_id).await
    }

    async fn init(&self) -> Result<OperationResult, OperationResult> {
        self.manager.init().await
    }

    fn unregister(&self) -> Result<OperationResult, OperationResult> {
        self.manager.unregister()
    }

    fn get_manager_id(&self) -> i32 {
        self.manager.get_manager_id()
    }

    fn get_manager_name(&self) -> String {
        self.manager.get_manager_name()
    }

    fn has_cache(&self) -> bool {
        self.manager.has_cache()
    }

    async fn get_manage(&self) -> Arc<RwLock<Manage>> {
        self.manager.get_manage().await
    }

    async fn get_new_entity_id(&self) -> Option<i64> {
        self.manager.get_new_entity_id().await
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
