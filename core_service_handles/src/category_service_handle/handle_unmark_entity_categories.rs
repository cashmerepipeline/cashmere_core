use dependencies_sync::bson::doc;
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::tonic::async_trait;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::general_field_ids::*;
use managers::manager_trait::ManagerTrait;
use request_utils::request_account_context;

use dependencies_sync::tonic::{Request, Response, Status};

#[async_trait]
pub trait HandleUnmarkEntityCategories {
    async fn handle_unmark_entity_categories(
        &self,
        request: Request<UnmarkEntityCategoriesRequest>,
    ) -> Result<Response<UnmarkEntityCategoriesResponse>, Status> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_unmark_entity_categories)
            .await
    }
}

async fn validate_view_rules(
    request: Request<UnmarkEntityCategoriesRequest>,
) -> Result<Request<UnmarkEntityCategoriesRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = &request.get_ref().target_manage_id;
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
    request: Request<UnmarkEntityCategoriesRequest>,
) -> Result<Request<UnmarkEntityCategoriesRequest>, Status> {
    Ok(request)
}

async fn handle_unmark_entity_categories(
    request: Request<UnmarkEntityCategoriesRequest>,
) -> Result<Response<UnmarkEntityCategoriesResponse>, Status> {
    let (account_id, _groups, _role_group) = request_account_context(request.metadata())?;

    let target_manage_id = &request.get_ref().manage_id;
    let target_entity_id = &request.get_ref().entity_id;
    let tag_ids = &request.get_ref().category_ids;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(target_manage_id).unwrap();

    let query_doc = doc! {
        ID_FIELD_ID.to_string():target_entity_id
    };
    let modify_doc = doc! {
        TAGS_FIELD_ID.to_string():{"$in":tag_ids.clone()},
    };

    let result = manager
        .remove_from_array_field(query_doc, modify_doc, &account_id)
        .await;

    match result {
        Ok(_r) => Ok(Response::new(UnmarkEntityCategoriesResponse {
            result: "ok".to_string(),
        })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}


