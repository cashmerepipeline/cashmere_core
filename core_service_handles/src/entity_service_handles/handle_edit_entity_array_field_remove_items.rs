use dependencies_sync::bson::{doc, Document};
use dependencies_sync::tonic::async_trait;
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::prost::bytes::Buf;
use dependencies_sync::tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;

use manage_define::general_field_ids::*;

use managers::traits::ManagerTrait;
use request_utils::request_account_context;



use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleEditEntityArrayFieldRemoveItems {
    /// 编辑区域
    async fn handle_edit_entity_array_field_remove_items(
        &self,
        request: Request<EditEntityArrayFieldRemoveItemsRequest>,
    ) -> UnaryResponseResult<EditEntityArrayFieldRemoveItemsResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_edit_entity_array_field_remove_items)
            .await
    }
}

async fn validate_view_rules(
    request: Request<EditEntityArrayFieldRemoveItemsRequest>,
) -> Result<Request<EditEntityArrayFieldRemoveItemsRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = &request.get_ref().manage_id;
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
    request: Request<EditEntityArrayFieldRemoveItemsRequest>,
) -> Result<Request<EditEntityArrayFieldRemoveItemsRequest>, Status> {
    Ok(request)
}

async fn handle_edit_entity_array_field_remove_items(
    request: Request<EditEntityArrayFieldRemoveItemsRequest>,
) -> UnaryResponseResult<EditEntityArrayFieldRemoveItemsResponse> {
    let (account_id, _groups, _role_group) = request_account_context(request.metadata());

    let manage_id = &request.get_ref().manage_id;
    let entity_id = &request.get_ref().entity_id;
    let field_id = &request.get_ref().field_id;
    // bson bytes {field_id:[items]}
    let items = &request.get_ref().items;

    let b_items = match Document::from_reader(items.reader()) {
        Ok(v) => {
            let t_v = v.get_array(field_id);
            if let Ok(r) = t_v {
                r.clone()
            } else {
                return Err(Status::data_loss("新值不能为空"));
            }
        }
        Err(_) => return Err(Status::data_loss("新值不能为空")),
    };

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(*manage_id).unwrap();

    let query_doc = doc! {
        ID_FIELD_ID.to_string():entity_id,
    };

    let mut modify_doc = Document::new();
    modify_doc.insert(field_id, doc! {"$in":b_items.clone()});

    let result = manager
        .remove_from_array_field(query_doc, modify_doc, &account_id)
        .await;

    match result {
        Ok(_r) => Ok(Response::new(EditEntityArrayFieldRemoveItemsResponse {
            result: "ok".to_string(),
        })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
