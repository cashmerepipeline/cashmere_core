use dependencies_sync::{bson::doc, futures::TryFutureExt, tonic::async_trait};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use managers::utils::make_new_entity_document;
use request_utils::request_account_context;

use dependencies_sync::tonic::{Request, Response, Status};

use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleNewArea {
    async fn handle_new_area(
        &self,
        request: Request<NewAreaRequest>,
    ) -> UnaryResponseResult<NewAreaResponse> {
        validate_view_rules(request)
        .and_then(validate_request_params)
        .and_then(handle_new_area).await
    }
}

async fn validate_view_rules(
    request: Request<NewAreaRequest>,
) -> Result<Request<NewAreaRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = AREAS_MANAGE_ID;
        let (_account_id, _groups, role_group) = request_account_context(request.metadata());
        if let Err(e) = view::validates::validate_collection_can_write(&manage_id, &role_group).await {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<NewAreaRequest>,
) -> Result<Request<NewAreaRequest>, Status> {
    Ok(request)
}


async fn handle_new_area(request: Request<NewAreaRequest>) -> UnaryResponseResult<NewAreaResponse> {
    let (account_id, _groups, role_group) = request_account_context(request.metadata());

    let parent_id = &request.get_ref().parent_id;
    let name = &request.get_ref().name;
    let level = &request.get_ref().level;
    let code = &request.get_ref().code;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(AREAS_MANAGE_ID).unwrap();

    let local_name = match name {
        Some(n) => n,
        None => {
            return Err(Status::aborted(format!("没有指定名称--{}", code)));
        }
    };

    let name_doc = doc! {local_name.language.clone():local_name.name.clone()};

    // 区域是否存在，存在则返回
    if manager
        .entity_exists(&doc! {
            NAME_MAP_FIELD_ID.to_string():name_doc.clone(),
        })
        .await
    {
        return Err(Status::aborted("区域已经存在"));
    }

    if let Some(mut new_entity_doc) = make_new_entity_document(&manager).await {
        new_entity_doc.insert("_id", code);
        new_entity_doc.insert(ID_FIELD_ID.to_string(), code);
        new_entity_doc.insert(NAME_MAP_FIELD_ID.to_string(), name_doc);
        new_entity_doc.insert(AREAS_PARENT_ID_FIELD_ID.to_string(), parent_id);
        new_entity_doc.insert(AREAS_LEVEL_FIELD_ID.to_string(), level);

        let result = manager
            .sink_entity(&mut new_entity_doc, &account_id, &role_group)
            .await;

        match result {
            Ok(_r) => Ok(Response::new(NewAreaResponse {
                result: code.to_string(),
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    } else {
        Err(Status::aborted("创建新区域失败"))
    }
}
