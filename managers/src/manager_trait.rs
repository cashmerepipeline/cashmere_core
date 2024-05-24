
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

use dependencies_sync::bson;
use dependencies_sync::bson::{doc, Document};
use dependencies_sync::log;

use dependencies_sync::parking_lot::RwLock;



use dependencies_sync::tonic::async_trait;


use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;






use crate::manage_interface::init_check;
use cash_core::schema_field_exists;

/// 管理接口
#[async_trait]
pub trait ManagerInterface: Any + Send + Sync {
    // 注册管理器
    async fn init_check(&self) -> Result<OperationResult, OperationResult> {
        let manage_id = &self.get_id().to_string();
        init_check(manage_id).await
    }

    // 移除管理器
    fn unregister(&self) -> Result<OperationResult, OperationResult>;

    // 取得管理器实体
    // fn get_manager() -> &'static Self;

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

    // /// zh: 管理是否硬编码
    // async fn is_hard_coded(&self) -> bool {
    //     let manage_arc = self.get_manage().await;
    //     let manage_rlock = manage_arc.read();

    //     debug!("{:?}", manage_rlock.hard_coded);
    //     return manage_rlock.hard_coded;
    // }

    // ---------------------------
    //  数据验证
    // ---------------------------

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
            if ks.is_empty() || ks.is_empty() {
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

    // async fn validate_data_fields_doc(
    //     &self,
    //     fields_doc: &Document,
    // ) -> Result<OperationResult, OperationResult> {
    //     // 取出ids
    //     let data_keys: Vec<i32> = fields_doc
    //         .iter()
    //         .map(|x| x.0.parse::<i32>().unwrap())
    //         .collect();

    //     // 检查是否在描写中
    //     let schema = self.get_manage_schema().await;

    //     for k in data_keys {
    //         if schema.iter().map(|x| x.id).any(|x| x == k) {
    //             continue;
    //         } else {
    //             return Err(operation_failed(
    //                 "validate_data_fields",
    //                 format!("属性不在描写中 {}", k),
    //             ));
    //         }
    //     }
    //     Ok(operation_succeed("ok"))
    // }

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
}
