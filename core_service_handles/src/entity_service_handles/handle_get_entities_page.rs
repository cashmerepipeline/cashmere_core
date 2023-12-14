use configs::ConfigTrait;
use configs::ServerConfigs;

use dependencies_sync::bson::{self, doc, Document};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::tokio;
use dependencies_sync::tokio_stream::wrappers::ReceiverStream;
use dependencies_sync::tokio_stream::StreamExt;
use dependencies_sync::tonic::async_trait;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use managers::manager_trait::ManagerTrait;
use request_utils::request_account_context;

use dependencies_sync::tonic::{Request, Response, Status};

use service_utils::send_stream_response;
use service_utils::types::{ResponseStream, StreamResponseResult};
use validates::validate_manage_id;

#[async_trait]
pub trait HandleGetEntitiesPage {
    /// zh: 取得产品分页
    /// en: Get product page
    async fn handle_get_entities_page(
        &self,
        request: Request<GetEntitiesPageRequest>,
    ) -> StreamResponseResult<GetEntitiesPageResponse> {
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

    validate_manage_id(manage_id).await?;

    Ok(request)
}

async fn handle_get_entities_page(
    request: Request<GetEntitiesPageRequest>,
) -> StreamResponseResult<GetEntitiesPageResponse> {
    let (_account_id, _groups, _role_group) = request_account_context(request.metadata())?;

    let manage_id = &request.get_ref().manage_id;
    let match_doc = &request.get_ref().match_doc;
    let sort_doc = &request.get_ref().sort_doc;
    // 页码和起始点不同时起作用
    let page_index = &request.get_ref().page_index;
    let start_oid = &request.get_ref().start_oid;

    let match_doc: Document = bson::from_slice(match_doc).unwrap_or(Document::new());
    let sort_doc: Document = bson::from_slice(sort_doc).unwrap_or(Document::new());

    let skip_count = page_index * 20;
    let start_oid: Option<&str> = if start_oid.is_empty() {
        None
    } else {
        Some(start_oid.as_str())
    };

    let sort = if !sort_doc.is_empty() {
        let mut result = doc! {};
        sort_doc.iter().for_each(|(k, v)| {
            result.insert(k, v);
        });
        Some(result)
    } else {
        None
    };

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(manage_id.as_str()).unwrap();

    let doc_stream = manager
        .get_entity_stream(match_doc, None, sort, start_oid, skip_count)
        .await;

    // 创建返回流
    let (resp_tx, resp_rx) = tokio::sync::mpsc::channel(1);
    tokio::spawn(async move {
        if let Ok(mut d_stream) = doc_stream {
            let mut max_page_size = ServerConfigs::get().max_page_size.clone();
            while let Some(doc) = d_stream.next().await {
                let resp = GetEntitiesPageResponse {
                    entity: bson::to_vec(&doc).unwrap(),
                };

                send_stream_response(&resp_tx, resp).await;

                max_page_size -= 1;
                if max_page_size == 0 {
                    break;
                }
            }
        }
    });

    let resp_stream = ReceiverStream::new(resp_rx);

    Ok(Response::new(
        Box::pin(resp_stream) as ResponseStream<GetEntitiesPageResponse>
    ))
}
