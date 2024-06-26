use dependencies_sync::bson::{self, doc};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::log::{self};
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::async_trait;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::general_field_ids::REMOVED_FIELD_ID;
use managers::{entity_interface::EntityInterface};
use request_utils::request_account_context;

use dependencies_sync::tokio_stream::{self as stream, StreamExt};
use dependencies_sync::tonic::{Request, Response, Status};
use validates::{validate_manage_id};
use view::{self, can_field_read};

use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleGetEntity {
    /// 取得管理记录数量
    async fn handle_get_entity(
        &self,
        request: Request<GetEntityRequest>,
    ) -> UnaryResponseResult<GetEntityResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_get_entity)
            .await
    }
}

async fn validate_view_rules(
    request: Request<GetEntityRequest>,
) -> Result<Request<GetEntityRequest>, Status> {
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
    request: Request<GetEntityRequest>,
) -> Result<Request<GetEntityRequest>, Status> {
    let manage_id = &request.get_ref().manage_id;
    let _entity_id = &request.get_ref().entity_id;

    validate_manage_id(manage_id).await?;

    Ok(request)
}

async fn handle_get_entity(
    request: Request<GetEntityRequest>,
) -> Result<Response<GetEntityResponse>, Status> {
    let (_account_id, _groups, role_group) = request_account_context(request.metadata())?;

    let manage_id = &request.get_ref().manage_id;
    let entity_id = &request.get_ref().entity_id;
    let present_fields = &request.get_ref().present_fields;
    let no_present_fields = &request.get_ref().no_present_fields;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(manage_id.as_str()).unwrap();

    let mut presents = present_fields.clone();
    // 需要返回删除字段
    if !present_fields.is_empty() && !present_fields.contains(&REMOVED_FIELD_ID.to_string()) {
        presents.push(REMOVED_FIELD_ID.to_string());
    }

    let result = manager
        .get_entity_by_id(entity_id, &presents, no_present_fields)
        .await;


    match result {
        Ok(r) => {
            if r.get_bool(REMOVED_FIELD_ID.to_string()).unwrap_or(true) {
                Err(Status::invalid_argument(format!(
                    "{}: {}-{}, {}",
                    t!("实体已删除"),
                    manage_id,
                    entity_id,
                    "handle_get_entity",
                )))
            } else {
                // 字段可见性过滤
                let mut result_doc = doc!();
                let mut property_stream = stream::iter(r);
                while let Some((k, v)) = property_stream.next().await {
                    if !can_field_read(manage_id, &k, &role_group).await {
                        log::debug!("{}: {} {}-{}", t!("字段不可见"), role_group, manage_id, k);
                        continue;
                    }
                    result_doc.insert(k, v);
                }

                Ok(Response::new(GetEntityResponse {
                    entity: bson::to_vec(&result_doc).unwrap(),
                }))
            }
        }
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
