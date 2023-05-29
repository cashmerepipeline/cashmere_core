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


async fn validate_view_rules(
  request: Request<RemoveEntityTemplateRequest>,
) -> Result<Request<RemoveEntityTemplateRequest>, Status> {
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
  request: Request<RemoveEntityTemplateRequest>,
) -> Result<Request<RemoveEntityTemplateRequest>, Status> {
  Ok(request)
}