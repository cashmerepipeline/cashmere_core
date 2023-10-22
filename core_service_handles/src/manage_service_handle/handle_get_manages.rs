use dependencies_sync::bson::doc;
use dependencies_sync::tonic::async_trait;
use dependencies_sync::tonic::{Request, Response, Status};
use dependencies_sync::futures::TryFutureExt;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::general_field_ids::{ID_FIELD_ID, NAME_MAP_FIELD_ID};
use managers::manager_trait::ManagerTrait;
use request_utils::request_account_context;


#[async_trait]
pub trait HandleGetManages {
    /// 取得管理
    async fn handle_get_manages(
        &self,
        request: Request<GetManagesRequest>,
    ) -> Result<Response<GetManagesResponse>, Status> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_get_manages)
            .await
    }
}

async fn validate_view_rules(
    request: Request<GetManagesRequest>,
) -> Result<Request<GetManagesRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let managers_ids: Vec<i32> = get_majordomo().get_manager_ids();
        for manage_id in managers_ids {
            let (_account_id, _groups, role_group) = request_account_context(request.metadata());
            if let Err(e) =
                view::validates::validate_collection_can_write(&manage_id, &role_group).await
            {
                return Err(e);
            }
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<GetManagesRequest>,
) -> Result<Request<GetManagesRequest>, Status> {
    Ok(request)
}

async fn handle_get_manages(
    request: Request<GetManagesRequest>,
) -> Result<Response<GetManagesResponse>, Status> {
    let (_account_id, _groups, _role_group) = request_account_context(request.metadata());

    let managers_ids: Vec<i32> = get_majordomo().get_manager_ids();

    let mut result: Vec<Manage> = Vec::new();
    for id in managers_ids {
        let manager = get_majordomo().get_manager_by_id(id).unwrap();
        let doc = manager.get_manage_document().await.read().clone();

        let mut name_map: Vec<u8> = Vec::new();
        doc.get_document(NAME_MAP_FIELD_ID.to_string())
            .unwrap()
            .to_writer(&mut name_map)
            .unwrap();

        let m = Manage {
            manage_id: doc
                .get_str(ID_FIELD_ID.to_string())
                .unwrap()
                .parse::<i32>()
                .unwrap(),
            name_map,
        };

        result.push(m);
    }

    Ok(Response::new(GetManagesResponse { manages: result }))
}
