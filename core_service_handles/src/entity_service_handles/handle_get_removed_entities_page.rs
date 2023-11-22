use dependencies_sync::bson::{self, doc, Document};
use dependencies_sync::tonic::async_trait;
use dependencies_sync::futures::TryFutureExt;

use majordomo::{self, get_majordomo};
use manage_define::{cashmere::*, general_field_ids::REMOVED_FIELD_ID};
use managers::manager_trait::ManagerTrait;
use request_utils::request_account_context;

use dependencies_sync::tonic::{Request, Response, Status};
use view::{add_query_filters, get_manage_schema_view_mask};

use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleGetRemovedEntitiesPage {
    /// 取得产品分页
    async fn handle_get_removed_entities_page(
        &self,
        request: Request<GetRemovedEntitiesPageRequest>,
    ) -> UnaryResponseResult<GetRemovedEntitiesPageResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_get_removed_entities_page)
            .await
    }
}

async fn validate_view_rules(
    request: Request<GetRemovedEntitiesPageRequest>,
) -> Result<Request<GetRemovedEntitiesPageRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = &request.get_ref().manage_id;
        let (_account_id, _groups, role_group) = request_account_context(request.metadata())?;
        if let Err(e) =
            view::validates::validate_collection_can_write(&manage_id, &role_group).await
        {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<GetRemovedEntitiesPageRequest>,
) -> Result<Request<GetRemovedEntitiesPageRequest>, Status> {
    Ok(request)
}

async fn handle_get_removed_entities_page(
    request: Request<GetRemovedEntitiesPageRequest>,
) -> Result<Response<GetRemovedEntitiesPageResponse>, Status> {
    let (account_id, _groups, role_group) = request_account_context(request.metadata())?;

    let manage_id = &request.get_ref().manage_id;
    let page_index = &request.get_ref().page_index;
    let conditions = &request.get_ref().conditions;

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

    // 加入移除标记过滤
    matches.insert(REMOVED_FIELD_ID.to_string(), true);

    // zh: 描写字段可见性过滤, 加入mongodb的project方法
    let fields = manager.get_manage_schema().await;
    let unsets =
        get_manage_schema_view_mask(&manage_id.to_string(), &fields, &role_group).await
        .iter()
        .filter(|(k, v)| **v == false)
        .map(|(k, v)| k.clone())
        .collect();
    

    // zh: 从1开始，
    let index = if *page_index == 0u32 {
        1u32
    } else {
        *page_index
    };

    let result = manager
        .get_entities_by_page(index, &Some(matches), &sorts_doc, &unsets)
        .await;

    match result {
        Ok(entities) => Ok(Response::new(GetRemovedEntitiesPageResponse {
            entities: entities.iter().map(|x| bson::to_vec(x).unwrap()).collect(),
        })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
