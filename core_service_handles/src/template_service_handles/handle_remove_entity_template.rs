use dependencies_sync::{tokio, tokio::sync::mpsc};

use dependencies_sync::tokio_stream::{wrappers::ReceiverStream, StreamExt};
use dependencies_sync::tonic::{Response, Status};

use dependencies_sync::tonic::async_trait;
use dependencies_sync::log::info;

use data_server::file_utils::get_chunk_md5;
use manage_define::cashmere::*;
use manage_define::manage_ids::*;
use request_utils::request_account_context;
use view;

use service_utils::types::{RequestStream, ResponseStream, StreamResponseResult};


#[async_trait]
pub trait HandleRemoveEntityTemplate {
    // 实体模板
    async fn handle_remove_entity_template(
        &self,
        request: Request<NewEntityTemplateRequest>,
    ) -> Result<Response<NewEntityTemplateResponse>, Status> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_remove_entity_template)
            .await
    }
}

async fn validate_view_rules(
  request: Request<RemoveEntityTemplateRequest>,
) -> Result<Request<RemoveEntityTemplateRequest>, Status> {
  #[cfg(feature = "view_rules_validate")]
  {
      let manage_id = ENTITIY_TEMPLATES_MANAGE_ID;
      let (_account_id, _groups, role_group) = request_account_context(request.metadata())?;
      if let Err(e) = view::validates::validate_collection_can_write(&manage_id, &role_group).await {
          return Err(e);
      }
  }

  Ok(request)
}

async fn validate_request_params(
  request: Request<RemoveEntityTemplateRequest>,
) -> Result<Request<RemoveEntityTemplateRequest>, Status> {
  Ok(request)
}

async fn handle_remove_entity_template(
  request: Request<RemoveEntityTemplateRequest>,
) -> Result<Response<RemoveEntityTemplateResponse>, Status> {
  let (account_id, _groups, role_group) = request_account_context(request.metadata())?;

  let target_manage_id = &request.get_ref().target_manage_id;
  let target_entity_id = &request.get_ref().target_entity_id;

  let majordomo_arc = get_majordomo();
  let entity_template_manager = majordomo_arc
      .get_manager_by_id(ENTITIY_TEMPLATES_MANAGE_ID)
      .unwrap();

  let result = entity_template_manager.mark_entity_removed(target_manage_id, target_entity_id).await;
  match result {
      Ok(_) => {
          let response = RemoveEntityTemplateResponse {};
          Ok(Response::new(response))
      }
      Err(e) => Err(Status::internal(e.to_string())),
  }
}