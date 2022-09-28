use async_trait::async_trait;
use bson::{doc, Document};
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use managers::traits::ManagerTrait;
use view::{add_query_filters, get_manage_schema_view};

use crate::UnaryResponseResult;

#[async_trait]
pub trait HandleGetEntitiesPage {
    /// 取得产品分页
    async fn handle_get_entities_page(
        &self,
        request: Request<GetEntitiesPageRequest>,
    ) -> UnaryResponseResult<GetEntitiesPageResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let manage_id = &request.get_ref().manage_id;
        let page_index = &request.get_ref().page_index;
        let conditions = &request.get_ref().conditions;

        // 管理可读性检查
        if !view::can_manage_read(&account_id, &role_group, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有管理可读权限"));
        }

        // 集合可读性检查
        if !view::can_collection_read(&account_id, &role_group, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有集合可读权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc.get_manager_by_id(*manage_id).await.unwrap();

        // TODO: 排序条件只支持几种固定格式，需要安全性检查
        let sorts_doc = bson::to_document(conditions).ok().or(None);

        // 可读性过滤, 没有设置过滤即不可读
        // TODO: 根据组改写，加入可读过滤项
        let mut matches = doc! {};
        if let Some(filter_doc) =
            add_query_filters(&account_id.to_string(), &role_group, &manage_id.to_string()).await
        {
            filter_doc.iter().for_each(|(k, v)| {
                matches.insert(k, v);
            });
        } else {
            return Err(Status::unauthenticated(
                "没有可读描写字段，用户不具有集合可读权限",
            ));
        };

        // 描写字段可见性过滤, 加入mongodb的project方法
        let fields = manager.get_manage_schema().await;
        let schema_projects =
            get_manage_schema_view(&account_id, &role_group, &manage_id.to_string(), &fields).await;

        let project_doc = if schema_projects.len() > 0 {
            // 只加入不可见字段
            let mut no_show_project = Document::new();
            schema_projects.iter().for_each(|(k, v)| {
                if v.as_i32().unwrap() == 0 {
                    no_show_project.insert(k, v);
                }
            });
            Some(no_show_project)
        } else {
            None
        };

        // 从0开始，
        // TODO：需要验证数据库起始编号
        let index = if *page_index <= 0u32 {
            0u32
        } else {
            *page_index - 1
        };

        let result = manager
            .get_entities_by_page(index, &Some(matches), &sorts_doc, &project_doc)
            .await;

        match result {
            Ok(entities) => Ok(Response::new(GetEntitiesPageResponse {
                entities: entities
                    .iter()
                    .map(|x| {
                        let mut bytes: Vec<u8> = Vec::new();
                        x.to_writer(&mut bytes).expect(&*format!("数据损坏:{}", x));
                        bytes
                    })
                    .collect(),
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}
