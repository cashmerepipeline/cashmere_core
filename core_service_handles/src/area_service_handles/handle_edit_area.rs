use dependencies_sync::bson::doc;
use dependencies_sync::tonic::async_trait;
use dependencies_sync::futures::TryFutureExt;

use dependencies_sync::tonic::{Request, Response, Status};
use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;
use manage_define::manage_ids::*;
use managers::manager_trait::ManagerTrait;
use request_utils::request_account_context;


use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleEditArea {
    /// 编辑区域
    async fn handle_edit_area(
        &self,
        request: Request<EditAreaRequest>,
    ) -> UnaryResponseResult<EditAreaResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_edit_area)
            .await
    }
}

async fn validate_view_rules(
    request: Request<EditAreaRequest>,
) -> Result<Request<EditAreaRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = AREAS_MANAGE_ID;
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
    request: Request<EditAreaRequest>,
) -> Result<Request<EditAreaRequest>, Status> {
    Ok(request)
}

async fn handle_edit_area(
    request: Request<EditAreaRequest>,
) -> Result<Response<EditAreaResponse>, Status> {
    let (account_id, _groups, _role_group) = request_account_context(request.metadata());

    let area_id = &request.get_ref().area_id;
    let new_parent_id = &request.get_ref().new_parent_id;
    let new_level = &request.get_ref().new_level;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(AREAS_MANAGE_ID).unwrap();

    let query_doc = doc! {
        "_id": area_id
    };
    let mut modify_doc = doc! {
        AREAS_PARENT_ID_FIELD_ID.to_string(): new_parent_id,
        AREAS_LEVEL_FIELD_ID.to_string():new_level
    };

    let result = manager
        .update_entity_field(query_doc, &mut modify_doc, &account_id)
        .await;

    match result {
        Ok(_r) => Ok(Response::new(EditAreaResponse {
            result: "ok".to_string(),
        })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
