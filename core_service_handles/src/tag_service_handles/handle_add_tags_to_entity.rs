use dependencies_sync::bson::doc;
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::tonic::async_trait;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::general_field_ids::*;
use managers::traits::ManagerTrait;
use request_utils::request_account_context;

use dependencies_sync::tonic::{Request, Response, Status};

use crate::tag_service_handles;

#[async_trait]
pub trait HandleAddTagsToEntity {
    async fn handle_add_tags_to_entity(
        &self,
        request: Request<AddTagsToEntityRequest>,
    ) -> Result<Response<AddTagsToEntityResponse>, Status> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_add_tags_to_entity)
            .await
    }
}

async fn validate_view_rules(
    request: Request<AddTagsToEntityRequest>,
) -> Result<Request<AddTagsToEntityRequest>, Status> {
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
    request: Request<AddTagsToEntityRequest>,
) -> Result<Request<AddTagsToEntityRequest>, Status> {
    Ok(request)
}

async fn handle_add_tags_to_entity(
    request: Request<AddTagsToEntityRequest>,
) -> Result<Response<AddTagsToEntityResponse>, Status> {
    let (account_id, _groups, _role_group) = request_account_context(request.metadata());

    let target_manage_id = &request.get_ref().target_manage_id;
    let target_entity_id = &request.get_ref().target_entity_id;
    let tag_ids = &request.get_ref().tag_ids;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(*target_manage_id).unwrap();

    let query_doc = doc! {
        ID_FIELD_ID.to_string():target_entity_id
    };
    let modify_doc = doc! {
        TAGS_FIELD_ID.to_string():{"$each":tag_ids.clone()},
    };

    let result = manager
        .add_to_array_field(query_doc, modify_doc, &account_id)
        .await;

    match result {
        Ok(_r) => Ok(Response::new(AddTagsToEntityResponse {
            result: "ok".to_string(),
        })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
