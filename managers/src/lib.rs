/*
Author: 闫刚
lib.rs (c) 2020
Desc: 大管家
Created:  2020-11-16T11:08:01.884Z
Modified: !date!
*/

// 异步线程限制
#![recursion_limit = "256"]

pub mod manages_manager;
pub mod accounts_manager;
pub mod groups_manager;
pub mod events_manager;
pub mod event_queues_manager;
pub mod event_handles_manager;
pub mod works_manager;
pub mod procedures_manager;
pub mod work_nodes_manager;
pub mod tasks_manager;
pub mod datas_manager;
pub mod areas_manager;
pub mod messages_manager;
pub mod message_handles_manager;
pub mod persons_manager;
pub mod show_settings_manager;
pub mod view_rules_manager;
pub mod phase_sets_manager;
pub mod templates_manager;
// mod manage_documents_map;
// mod manages_map;

pub mod points_manager;
pub mod graphs_manager;
pub mod orgnizations_manager;
pub mod departments_manager;
pub mod classes_manager;
pub mod courses_manager;
pub mod domains_manager;
pub mod questions_manager;
pub mod comments_manager;
pub mod answers_manager;
pub mod roadmaps_manager;
pub mod tag_types_manager;

pub mod traits;
mod macros;
pub(crate) mod schema;

use std::sync::{Arc};
use std::marker::{Sync, Send};

use bson::{Document};
use parking_lot::RwLock;
use async_trait::async_trait;

use cash_result::*;
use cash_core::Manage;
use traits::ManagerTrait;
use property_field::PropertyField;
// use log::{info};

/// 管理器包装
#[derive(Clone)]
pub struct Manager {
    inner: Arc<ManagerInner>
}

#[derive(Clone)]
pub struct ManagerInner {
    manager: Arc<dyn ManagerTrait>
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

    async fn get_entity_by_id(&self, entity_id: &String) -> Result<Document, OperationResult> {
        self.inner.get_entity_by_id(entity_id).await
    }

    async fn get_entities_by_filter(&self, filter: &Option<Document>) -> Result<Vec<Document>, OperationResult> {
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
        self.manager.get_entity_by_id(&entity_id).await
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


/// 取得所有管理器
pub async fn get_managers() -> Vec<Arc<Manager>> {
    let manages_manager_arc = manages_manager::get_manager().await;
    let accounts_manager_arc = accounts_manager::get_manager().await;
    let datas_manager_arc = datas_manager::get_manager().await;
    let groups_manager_arc = groups_manager::get_manager().await;
    let local_codes_manager_arc = areas_manager::get_manager().await;
    let messages_manager_arc = messages_manager::get_manager().await;
    let message_handles_manager_arc = message_handles_manager::get_manager().await;
    let persons_manager_arc = persons_manager::get_manager().await;
    let phase_sets_manager_arc = phase_sets_manager::get_manager().await;
    let events_manager_arc = events_manager::get_manager().await;
    let event_handles_manager_arc = event_handles_manager::get_manager().await;
    let event_queues_manager_arc = event_queues_manager::get_manager().await;
    let works_manager_arc = works_manager::get_manager().await;
    let procedures_manager_arc = procedures_manager::get_manager().await;
    let work_nodes_manager_arc = work_nodes_manager::get_manager().await;
    let tasks_manager_arc = tasks_manager::get_manager().await;
    let show_settings_manager_arc = show_settings_manager::get_manager().await;
    let view_rules_manager_arc = view_rules_manager::get_manager().await;
    let templates_manager_arc = templates_manager::get_manager().await;

    vec![manages_manager_arc,
         templates_manager_arc,
         accounts_manager_arc,
         groups_manager_arc,
         local_codes_manager_arc,
         persons_manager_arc,
         view_rules_manager_arc,
         show_settings_manager_arc,
         messages_manager_arc,
         message_handles_manager_arc,
         works_manager_arc,
         phase_sets_manager_arc,
         procedures_manager_arc,
         work_nodes_manager_arc,
         tasks_manager_arc,
         datas_manager_arc,
         events_manager_arc,
         event_handles_manager_arc,
         event_queues_manager_arc,
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
