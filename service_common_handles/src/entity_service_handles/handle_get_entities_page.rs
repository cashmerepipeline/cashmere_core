use dependencies_sync::tonic::async_trait;
use dependencies_sync::bson::{self, doc, Document};

use majordomo::{self, get_majordomo};
use manage_define::{cashmere::*, general_field_ids::ENTITY_REMOVED_FIELD_ID};
use managers::traits::ManagerTrait;
use request_utils::request_account_context;

use dependencies_sync::tonic::{Request, Response, Status};

use view::{add_query_filters, get_manage_schema_view};

use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleGetEntitiesPage {
    /// 取得产品分页
    async fn handle_get_entities_page(
        &self,
        request: Request<GetEntitiesPageRequest>,
    ) -> UnaryResponseResult<GetEntitiesPageResponse> {
        let (account_id, _groups, role_group) =
            request_account_context(request.metadata());

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

        let majordomo_arc = get_majordomo();
        let manager = majordomo_arc.get_manager_by_id(*manage_id).unwrap();

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

        matches.insert(ENTITY_REMOVED_FIELD_ID.to_string(), false);

        // zh: 描写字段可见性过滤, 加入mongodb的project方法
        let fields = manager.get_manage_schema().await;
        let schema_projects =
            get_manage_schema_view(&account_id, &role_group, &manage_id.to_string(), &fields).await;
        let project_doc = if !schema_projects.is_empty() {
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

        // zh: 从1开始，
        let index = if *page_index == 0u32 {
            1u32
        } else {
            *page_index
        };

        let result = manager
            .get_entities_by_page(index, &Some(matches), &sorts_doc, &project_doc)
            .await;

        match result {
            Ok(entities) => Ok(Response::new(GetEntitiesPageResponse {
                entities: entities.iter().map(|x| bson::to_vec(x).unwrap()).collect(),
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}
