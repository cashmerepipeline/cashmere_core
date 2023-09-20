use dependencies_sync::bson::doc;
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::{Request, Response, Status};
use dependencies_sync::tonic::async_trait;
use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::general_field_ids::*;
use managers::traits::ManagerTrait;
use request_utils::request_account_context;

#[async_trait]
pub trait HandleMarkEntityCategories {
    async fn handle_mark_entity_categories(
        &self,
        request: Request<MarkEntityCategoriesRequest>,
    ) -> Result<Response<MarkEntityCategoriesResponse>, Status> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_mark_entity_categories)
            .await
    }
}

async fn validate_view_rules(
    request: Request<MarkEntityCategoriesRequest>,
) -> Result<Request<MarkEntityCategoriesRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = &request.get_ref().target_manage_id;
        let (_account_id, _groups, role_group) = request_account_context(request.metadata());

        if let Err(e) =
            view::validates::validate_collection_can_write(&manage_id, &role_group).await
        {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<MarkEntityCategoriesRequest>,
) -> Result<Request<MarkEntityCategoriesRequest>, Status> {
    let target_manage_id = request.get_ref().manage_id;
    let target_entity_id = &request.get_ref().entity_id;
    let category_ids = &request.get_ref().category_ids;

    if target_manage_id == 0i32 {
        return Err(Status::invalid_argument(t!("管理编号不正确")));
    }
    if target_entity_id.is_empty() {
        return Err(Status::invalid_argument(t!("实体编号不能为空")));
    }

    if category_ids.is_empty() {
        return Err(Status::invalid_argument(t!("类别编号不能为空")));
    }

    Ok(request)
}

async fn handle_mark_entity_categories(
    request: Request<MarkEntityCategoriesRequest>,
) -> Result<Response<MarkEntityCategoriesResponse>, Status> {
    let (account_id, _groups, _role_group) = request_account_context(request.metadata());

    let target_manage_id = request.get_ref().manage_id;
    let target_entity_id = &request.get_ref().entity_id;
    let category_ids = &request.get_ref().category_ids;

    let majordomo_arc = get_majordomo();

    // 目标管理是否存在
    if majordomo_arc.get_manager_by_id(target_manage_id).is_err() {
        return Err(Status::not_found(format!(
            "{}-{}",
            t!("管理不存在"),
            target_entity_id
        )));
    };

    let manager = majordomo_arc.get_manager_by_id(target_manage_id).unwrap();

    let query_doc = doc! {
        ID_FIELD_ID.to_string():target_entity_id
    };
    let modify_doc = doc! {
        CATEGORIES_FIELD_ID.to_string(): {"$each": category_ids.clone() },
    };

    let result = manager
        .add_to_array_field(query_doc, modify_doc, &account_id)
        .await;

    match result {
        Ok(_r) => Ok(Response::new(MarkEntityCategoriesResponse {
            result: "ok".to_string(),
        })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
