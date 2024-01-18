/*
Project: cashmere_server
Creator: 闫刚
Create time: 2020-11-16 16:20
Introduction:
*/
use std::{any::Any, sync::Arc};

use cash_core::Manage;
use cash_core::SchemaField;
use cash_result::*;
use database;
use dependencies_sync::bson;
use dependencies_sync::bson::{doc, Document};
use dependencies_sync::log;
use dependencies_sync::parking_lot::RwLock;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tokio;
use dependencies_sync::tokio::sync::mpsc;
use dependencies_sync::tokio_stream::wrappers::ReceiverStream;
use dependencies_sync::tokio_stream::StreamExt;
use dependencies_sync::tonic::async_trait;
use entity;
use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;

use crate::entity_cache_map::cache_get_entity;
use crate::entity_cache_map::{cache_get_entity_stream, cache_init_cache, cache_update_entity};
use crate::entity_interface;
use crate::entity_interface::get_entities_by_filter;
use crate::entity_interface::get_entity_by_id;
use crate::entity_interface::get_entity_stream;
use crate::manage_interface;
use crate::manage_interface::init_check;
use cash_core::schema_field_exists;

/// 管理接口
#[async_trait]
pub trait ManagerTrait: Any + Send + Sync {
    // 注册管理器
    async fn init_check(&self) -> Result<OperationResult, OperationResult> {
        let manage_id = &self.get_id().to_string();
        init_check(manage_id).await
    }

    // 移除管理器
    fn unregister(&self) -> Result<OperationResult, OperationResult>;

    // 取得管理器id
    fn get_id(&self) -> &'static str;
    // 取得管理器名
    fn get_name(&self) -> String;

    // 取得管理实体
    async fn get_manage(&self) -> Arc<RwLock<Manage>>;

    // 取得管理数据document
    async fn get_manage_document(&self) -> Arc<RwLock<Document>>;

    // ---------------------------
    //  管理描写模式
    // ---------------------------
    // 取得管理描写, 如果为空则返回空表，无异常发生
    async fn get_manage_schema(&self) -> Vec<SchemaField> {
        let manage_lock = self.get_manage().await;
        let manage = manage_lock.read();
        if manage.schema.is_empty() {
            vec![]
        } else {
            manage.schema.clone()
        }
    }

    // 取得描写模式bson
    async fn get_schema_document(&self) -> Result<Document, OperationResult> {
        let manage_doc_arc = self.get_manage_document().await;
        let manage_doc_rlock = manage_doc_arc.read();
        Ok(manage_doc_rlock
            .get_document(&MANAGES_SCHEMA_FIELD_ID.to_string())
            .unwrap()
            .clone())
    }

    async fn has_schema_field(&self, field_id: i32) -> bool {
        let schema = self
            .get_manage_schema()
            .await
            .iter()
            .map(|x| x.id)
            .collect::<Vec<i32>>();
        schema.contains(&field_id)
    }

    // ---------------------------
    //  数据验证
    // ---------------------------
    async fn validate_data_field(
        &self,
        data: &Vec<u8>,
    ) -> Result<OperationResult, OperationResult> {
        let d = if let Ok(d) = bson::from_slice::<Document>(data) {
            d
        } else {
            return Err(operation_failed(
                "validate_data_fields",
                "数据不是数据对的形式",
            ));
        };

        // let b = data.clone();
        // let d = match Document::from_reader(&mut b.as_slice()) {
        //     Ok(r) => r,
        //     Err(_e) => {
        //         return Err(operation_failed(
        //             "validate_data_fields",
        //             "数据不是数据对的形式",
        //         ));
        //     }
        // };
        let ks: Vec<i32> = d.keys().map(|x| x.parse::<i32>().unwrap()).collect();
        if ks.is_empty() || ks.len() > 1 {
            return Err(operation_failed("validate_data_fields", "数据格式不正确"));
        }
        // 检查是否在描写中
        let schema = self.get_manage_schema().await;

        if schema.iter().map(|x| x.id).any(|x| x == ks[0]) {
            Ok(operation_succeed("ok"))
        } else {
            return Err(operation_failed(
                "validate_data_fields",
                format!("属性不在描写模式中 {}", ks[0]),
            ));
        }
    }

    async fn validate_data_fields(
        &self,
        data: &Vec<Vec<u8>>,
    ) -> Result<OperationResult, OperationResult> {
        // 取出ids
        let mut data_keys: Vec<i32> = Vec::new();
        for x in data {
            let b = x.clone();
            let d = match Document::from_reader(&mut b.as_slice()) {
                Ok(r) => r,
                Err(_e) => {
                    return Err(operation_failed(
                        "validate_data_fields",
                        "数据不是数据对的形式",
                    ));
                }
            };
            let ks: Vec<i32> = d.keys().map(|x| x.parse::<i32>().unwrap()).collect();
            if ks.is_empty() || ks.len() > 1 {
                return Err(operation_failed("validate_data_fields", "数据格式不正确"));
            }

            data_keys.push(ks[0]);
        }
        // 检查是否在描写中
        let schema = self.get_manage_schema().await;

        for k in data_keys {
            if schema.iter().map(|x| x.id).any(|x| x == k) {
                continue;
            } else {
                return Err(operation_failed(
                    "validate_data_fields",
                    format!("属性不在描写中 {}", k),
                ));
            }
        }
        Ok(operation_succeed("ok"))
    }

    async fn validate_data_fields_doc(
        &self,
        fields_doc: &Document,
    ) -> Result<OperationResult, OperationResult> {
        // 取出ids
        let data_keys: Vec<i32> = fields_doc
            .iter()
            .map(|x| x.0.parse::<i32>().unwrap())
            .collect();

        // 检查是否在描写中
        let schema = self.get_manage_schema().await;

        for k in data_keys {
            if schema.iter().map(|x| x.id).any(|x| x == k) {
                continue;
            } else {
                return Err(operation_failed(
                    "validate_data_fields",
                    format!("属性不在描写中 {}", k),
                ));
            }
        }
        Ok(operation_succeed("ok"))
    }

    // 取得管理描写二进制列表
    async fn get_manage_schema_bytes(&self) -> Result<Vec<Vec<u8>>, OperationResult> {
        let mut m_docs_op: Option<Vec<Vec<u8>>> = None;
        {
            let doc_arc = self.get_manage_document().await;
            let doc = doc_arc.read();

            // println!("{:?}", doc_r);
            let schema = match doc.get_array(&MANAGES_SCHEMA_FIELD_ID.to_string()) {
                Ok(r) => r.clone(),
                Err(_e) => {
                    return Err(operation_failed(
                        "get_manage_schema_bytes",
                        "取得描写数据失败",
                    ));
                }
            };

            let data: Vec<Vec<u8>> = schema
                .iter()
                .map(|x| {
                    let d = x.as_document().unwrap().clone();
                    let mut data: Vec<u8> = Vec::new();
                    d.to_writer(&mut data).unwrap();
                    data
                })
                .collect();

            m_docs_op.replace(data);
        }

        Ok(m_docs_op.unwrap())
    }

    /// 新建管理描写属性
    async fn new_schema_field(
        &self,
        new_field: SchemaField,
        account_id: &str,
    ) -> Result<(), OperationResult> {
        let field_id = new_field.id;

        // 更新管理
        let manage_id = self.get_id();
        {
            let manage_arc = self.get_manage().await;
            let mut manage = manage_arc.write();

            // 检查是否存着，不存在则添加，缓存和数据库不再检查
            if schema_field_exists(field_id, &manage.schema) {
                return Err(target_already_exists(
                    &field_id.to_string(),
                    "new_schema_field",
                ));
            } else {
                manage.schema.push(new_field.clone());
            }
        }

        //  更新数据缓存
        {
            let doc_arc = self.get_manage_document().await;
            let mut doc = doc_arc.write();
            let schema = doc
                .get_array_mut(&MANAGES_SCHEMA_FIELD_ID.to_string())
                .unwrap();
            let new_bson = bson::to_bson(&new_field).unwrap();
            schema.push(new_bson);
        }

        // 更新数据库
        let value = bson::to_bson(&new_field).unwrap();
        let query_doc = doc! {
            ID_FIELD_ID.to_string(): manage_id.to_string()
        };
        let modify_doc = doc! {
            MANAGES_SCHEMA_FIELD_ID.to_string():value
        };

        match entity::add_to_array_field(MANAGES_MANAGE_ID, query_doc, modify_doc, account_id).await
        {
            Err(e) => return Err(add_call_name_to_chain(e, "new_schema_field".to_string())),
            _ => Ok(()),
        }
    }

    async fn edit_schema_field_name(
        &self,
        field_id: i32,
        local: &String,
        new_name: &String,
        account_id: &str,
    ) -> Result<OperationResult, OperationResult> {
        // 更新管理
        let manage_id = self.get_id();
        let mut new_field: Option<SchemaField> = None;
        {
            let manage_arc = self.get_manage().await;
            let mut manage = manage_arc.write();

            // 字段是否存在
            if !manage.schema.iter().map(|x| x.id).any(|x| x == field_id) {
                return Err(field_not_exists(
                    "edit_schema_field_name",
                    field_id.to_string(),
                ));
            }

            let index = manage.schema.iter().position(|x| x.id == field_id).unwrap();
            let field = &mut manage.schema[index];

            // 名字已经存在，不需要更新
            if field.name_map.contains_key(local) && field.name_map.get(local).unwrap() == new_name
            {
                return Err(field_edited_already(
                    "edit_schema_field_name",
                    field_id.to_string(),
                ));
            }

            field.name_map.insert(local.clone(), new_name.clone());

            let field = manage.schema.get(index).unwrap().clone();
            new_field.replace(field);
        }
        let new_field = new_field.unwrap();

        // 更新数据库
        let value = bson::to_bson(&new_field).unwrap();

        let query_doc = doc! {
            "_id":manage_id.to_string(),
            format!("{}.id", MANAGES_SCHEMA_FIELD_ID):field_id
        };
        let modify_doc = doc! {
            format!("{}.$", MANAGES_SCHEMA_FIELD_ID): value
        };

        match entity::update_entity_array_element_field(
            MANAGES_MANAGE_ID,
            query_doc,
            modify_doc,
            account_id,
        )
        .await
        {
            Err(e) => return Err(add_call_name_to_chain(e, "new_schema_field".to_string())),
            _ => Ok(operation_succeed("ok")),
        }
    }

    async fn mark_schema_field_removed(
        &self,
        field_id: i32,
        account_id: &str,
    ) -> Result<OperationResult, OperationResult> {
        let manage_id = self.get_id();

        let _new_field: Option<SchemaField> = None;

        // 更新管理缓存
        {
            let manage_arc = self.get_manage().await;
            let mut manage = manage_arc.write();

            if !manage.schema.iter().map(|x| x.id).any(|x| x == field_id) {
                return Err(field_not_exists(
                    "edit_schema_field_name",
                    field_id.to_string(),
                ));
            }

            let index = manage.schema.iter().position(|x| x.id == field_id).unwrap();
            let field = &mut manage.schema[index];

            // 已经是removed
            if field.removed {
                // return Err(field_edited_already(
                //     "mark_schema_field_removed",
                //     field_id.to_string(),
                // ));
                return Ok(operation_succeed("ok"));
            }

            field.removed = true;
        }

        log::info!("update database of manage: {}", manage_id);

        // 更新数据库
        let query_doc = doc! {
            ID_FIELD_ID.to_string():manage_id.to_string(),
            format!("{}.id", MANAGES_SCHEMA_FIELD_ID):field_id
        };
        let modify_doc = doc! {
            format!("{}.$.removed", MANAGES_SCHEMA_FIELD_ID): true
        };

        if let Err(e) = entity::update_entity_array_element_field(
            MANAGES_MANAGE_ID,
            query_doc,
            modify_doc,
            account_id,
        )
        .await
        {
            return Err(add_call_name_to_chain(
                e,
                "mark_schema_field_removed".to_string(),
            ));
        };

        Ok(operation_succeed("ok"))
    }

    // ---------------------
    // 实体相关操作
    // ---------------------

    /// 实体是否可以删除，默认不可删除，使用removed字段
    fn is_entity_deleteable(&self) -> bool {
        false
    }

    // 实体缓存
    fn has_cache(&self) -> bool;

    async fn init_cache(&self) -> Result<OperationResult, OperationResult> {
        cache_init_cache(self.get_id()).await
    }

    // fn get_entities_cache_map(&self) -> Option<Arc<RwLock<HashMap<i32, Document>>>>;
    // fn refresh_cache(&self) -> Result<OperationResult, OperationResult>;

    // 直接替换缓存中的实体
    async fn update_cache(&self, new_doc: &Document) -> Result<OperationResult, OperationResult> {
        if !self.has_cache() {
            return Ok(operation_succeed("管理没有缓存，不需要更新"));
        }

        let id = if let Ok(r) = new_doc.get_str(ID_FIELD_ID.to_string()) {
            r.to_string()
        } else {
            return Err(operation_failed(
                "update_cache",
                t!("新实体没有编号, 格式不正确"),
            ));
        };

        cache_update_entity(self.get_id(), &id, new_doc.clone());

        Ok(operation_succeed(format!("{}: {}", t!("更新缓存完成"), id)))
    }

    async fn count_entity(&self, filter_doc: Document) -> Result<u64, OperationResult> {
        let manage_id = self.get_id();
        entity::count_entity(&manage_id.to_string(), filter_doc).await
    }

    async fn get_entry_counts(&self) -> Result<u64, OperationResult> {
        let manage_id = self.get_id();

        entity::get_entry_count(&manage_id.to_string()).await
    }

    /// 取得新实体id, 针对数量有限相对固定的管理使用, 不需要使用id的情况需要重写本方法
    async fn get_new_entity_id(&self, account_id: &str) -> Option<i64> {
        let manage_id = self.get_id().to_string();
        entity::get_new_entity_id(&manage_id.to_string(), &account_id).await
    }

    // 新建实体
    async fn sink_entity(
        &self,
        new_entity_doc: &mut Document,
        account_id: &str,
        group_id: &str,
    ) -> Result<String, OperationResult> {
        entity_interface::sink_entity(
            self.get_id(),
            new_entity_doc,
            account_id,
            group_id,
            self.has_cache(),
        )
        .await
    }

    /// 通过id取得实体
    async fn get_entity_by_id(
        &self,
        entity_id: &str,
        present_fields: &[String],
        no_present_fields: &[String],
    ) -> Result<Document, OperationResult> {
        let manage_id = self.get_id();
        get_entity_by_id(manage_id, entity_id, self.has_cache(), present_fields, no_present_fields).await
    }

    /// 通过过滤取得实体
    async fn get_entities_by_filter(
        &self,
        filter: &Option<Document>,
    ) -> Result<Vec<Document>, OperationResult> {
        let manage_id = self.get_id();
        get_entities_by_filter(manage_id, filter).await
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
        unsets: Option<Vec<String>>,
        sorts: Option<Document>,
        start_oid: Option<&str>,
        skip_count: u32,
    ) -> Result<ReceiverStream<Document>, OperationResult> {
        let manage_id = self.get_id();

        get_entity_stream(
            manage_id,
            matche_doc,
            unsets,
            sorts,
            start_oid,
            skip_count,
            self.has_cache(),
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
        match entity::get_entity_by_id(manage_id, entity_id, &[]).await {
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

    // ---------------------
    // 映像
    // 取得映像
    // async fn get_view(&self, account_id: &str, )

    // 关联事件队列

    // 触发事件

    // ---------------------
    // 消息
    // 新建消息

    // 关联消息队列

    // 发送消息

    // ---------------------
    // 数据
    // ---------------
}
