use dependencies_sync::bson::{self, Document, doc};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::log::error;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::async_trait;

use majordomo::{self, get_majordomo};
use manage_define::{cashmere::*, general_field_ids::REMOVED_FIELD_ID};
use managers::manager_trait::ManagerTrait;
use request_utils::request_account_context;

use dependencies_sync::tonic::{Request, Response, Status};

use service_utils::types::UnaryResponseResult;

use super::get_manage_entities_page;

#[async_trait]
pub trait HandleGetEntitiesPage {
    /// zh: 取得产品分页
    /// en: Get product page
    async fn handle_get_entities_page(
        &self,
        request: Request<GetEntitiesPageRequest>,
    ) -> UnaryResponseResult<GetEntitiesPageResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_get_entities_page)
            .await
    }
}

async fn validate_view_rules(
    request: Request<GetEntitiesPageRequest>,
) -> Result<Request<GetEntitiesPageRequest>, Status> {
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
    request: Request<GetEntitiesPageRequest>,
) -> Result<Request<GetEntitiesPageRequest>, Status> {
    let manage_id = &request.get_ref().manage_id;

    let majordomo_arc = get_majordomo();
    if majordomo_arc.get_manager_by_id(*manage_id).is_err() {
        error!("{} {}", t!("没有找到对应的管理器"), manage_id);

        return Err(Status::aborted(format!(
            "{} {}",
            t!("没有找到对应的管理器"),
            manage_id
        )));
    };

    Ok(request)
}

async fn handle_get_entities_page(
    request: Request<GetEntitiesPageRequest>,
) -> Result<Response<GetEntitiesPageResponse>, Status> {
    let (account_id, _groups, role_group) = request_account_context(request.metadata())?;

    let manage_id = &request.get_ref().manage_id;
    let page_index = &request.get_ref().page_index;
    let conditions = &request.get_ref().conditions;

    let sorts_doc: Document = bson::from_slice(&conditions).unwrap_or(Document::new());

    let result = get_manage_entities_page(
        &account_id, &role_group, manage_id, &doc!{}, &Some(sorts_doc), page_index,
    )
    .await;

    match result {
        Ok(entities) => Ok(Response::new(GetEntitiesPageResponse {
            entities: entities.iter().map(|x| bson::to_vec(&x).unwrap()).collect(),
        })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
