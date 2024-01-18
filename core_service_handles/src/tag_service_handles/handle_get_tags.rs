use dependencies_sync::bson::{self, doc};
use dependencies_sync::futures::TryFutureExt;

use dependencies_sync::tonic::async_trait;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;

use manage_define::manage_ids::TAGS_MANAGE_ID;
use managers::manager_trait::ManagerTrait;

use dependencies_sync::tokio_stream::StreamExt;
use dependencies_sync::tonic::{Request, Response, Status};

use service_utils::types::UnaryResponseResult;
use validates::validate_manage_id;

#[async_trait]
pub trait HandleGetTags {
    /// 取得管理记录数量
    async fn handle_get_tags(
        &self,
        request: Request<GetTagsRequest>,
    ) -> UnaryResponseResult<GetTagsResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_get_tags)
            .await
    }
}

async fn validate_view_rules(
    request: Request<GetTagsRequest>,
) -> Result<Request<GetTagsRequest>, Status> {
    // 全部可读
    Ok(request)
}

async fn validate_request_params(
    request: Request<GetTagsRequest>,
) -> Result<Request<GetTagsRequest>, Status> {
    // 没有参数
    let manage_id = &request.get_ref().target_manage_id;
    validate_manage_id(manage_id).await?;

    Ok(request)
}

async fn handle_get_tags(
    request: Request<GetTagsRequest>,
) -> Result<Response<GetTagsResponse>, Status> {
    let manager_id = TAGS_MANAGE_ID;

    let manage_id = &request.get_ref().target_manage_id;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(manager_id).unwrap();

    let query_doc = doc! {
        TAGS_TARGET_MANAGES_FIELD_ID.to_string():manage_id,
    };

    let result = manager
        .get_entity_stream(query_doc, None, None, None, 0)
        .await;

    match result {
        Ok(mut entities_iter) => {
            let mut results = vec![];
            while let Some(r) = entities_iter.next().await {
                results.push(bson::to_vec(&r).unwrap());
            }

            Ok(Response::new(GetTagsResponse { tags: results }))
        }
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
