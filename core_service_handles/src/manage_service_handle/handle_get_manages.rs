use dependencies_sync::bson::{self, doc};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::tonic::async_trait;
use dependencies_sync::tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::MANAGES_SCHEMA_FIELD_ID;
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
            let (_account_id, _groups, role_group) = request_account_context(request.metadata())?;
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
    let (_account_id, _groups, _role_group) = request_account_context(request.metadata())?;

    let managers_ids = get_majordomo()
        .get_manager_ids()
        .iter()
        .map(|id| id.to_string())
        .collect::<Vec<String>>();

    let mut result: Vec<Vec<u8>> = Vec::new();
    for id in managers_ids {
        let manager = get_majordomo().get_manager_by_id(id.as_str()).unwrap();
        let mut doc = manager.get_manage_document().await.read().clone();

        let _ = doc.remove(MANAGES_SCHEMA_FIELD_ID.to_string());

        result.push(bson::to_vec(&doc).unwrap());
    }

    Ok(Response::new(GetManagesResponse { manages: result }))
}
