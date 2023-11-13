use dependencies_sync::bson::{doc, Document};
use dependencies_sync::tonic::async_trait;
use dependencies_sync::futures::TryFutureExt;

use dependencies_sync::prost::bytes::Buf;

use dependencies_sync::tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;

use manage_define::general_field_ids::*;

use managers::manager_trait::ManagerTrait;
use request_utils::request_account_context;



use service_utils::types::UnaryResponseResult;

use super::{validate_edit_field_id, validate_edit_entity_id};

#[async_trait]
pub trait HandleEditEntityMapField {
    /// 编辑区域
    async fn handle_edit_entity_map_field(
        &self,
        request: Request<EditEntityMapFieldRequest>,
    ) -> UnaryResponseResult<EditEntityMapFieldResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_edit_entity_map_field)
            .await
    }
}

async fn validate_view_rules(
    request: Request<EditEntityMapFieldRequest>,
) -> Result<Request<EditEntityMapFieldRequest>, Status> {
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
    request: Request<EditEntityMapFieldRequest>,
) -> Result<Request<EditEntityMapFieldRequest>, Status> {
    let manage_id = &request.get_ref().manage_id;
    let entity_id = &request.get_ref().entity_id;
    let field_id = &request.get_ref().field_id;
    let key = &request.get_ref().key;

    if let Err(err) = validate_edit_entity_id(manage_id, entity_id).await {
        return Err(err);
    }

    let fields = match validate_edit_field_id(manage_id, entity_id, field_id).await {
        Ok(fields) => fields,
        Err(err) => return Err(err),
    };

    Ok(request)
}

async fn handle_edit_entity_map_field(
    request: Request<EditEntityMapFieldRequest>,
) -> UnaryResponseResult<EditEntityMapFieldResponse> {
    let (account_id, _groups, _role_group) = request_account_context(request.metadata());

    let manage_id = &request.get_ref().manage_id;
    let entity_id = &request.get_ref().entity_id;
    let field_id = &request.get_ref().field_id;
    let key = &request.get_ref().key;
    // bson bytes {key:new_value}
    let new_value = &request.get_ref().new_value;

    let value = match Document::from_reader(new_value.reader()) {
        Ok(ref v) => {
            // 属性key一致
            let t_v = v.get(key);
            if let Some(r) = t_v {
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
    modify_doc.insert(format!("{}.{}", field_id, key), value.clone());

    let result = manager
        .update_entity_field(query_doc, &mut modify_doc, &account_id)
        .await;

    match result {
        Ok(_r) => Ok(Response::new(EditEntityMapFieldResponse {
            result: "ok".to_string(),
        })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
