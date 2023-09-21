use dependencies_sync::bson::{self, doc};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::log::{error, info, debug};
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::async_trait;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;

use manage_define::general_field_ids::*;
use manage_define::manage_ids::{CATEGORIES_MANAGE_ID};
use managers::traits::ManagerTrait;

use dependencies_sync::tokio_stream::{self as stream, StreamExt};
use dependencies_sync::tonic::{Request, Response, Status};
use view::{self, can_entity_read, can_field_read};

use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleGetCategories {
    /// 取得管理记录数量
    async fn handle_get_categories(
        &self,
        request: Request<GetCategoriesRequest>,
    ) -> UnaryResponseResult<GetCategoriesResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_get_categories)
            .await
    }
}

async fn validate_view_rules(
    request: Request<GetCategoriesRequest>,
) -> Result<Request<GetCategoriesRequest>, Status> {
    // 全部可读
    Ok(request)
}

async fn validate_request_params(
    request: Request<GetCategoriesRequest>,
) -> Result<Request<GetCategoriesRequest>, Status> {
    // 没有参数
    Ok(request)
}

async fn handle_get_categories(
    request: Request<GetCategoriesRequest>,
) -> Result<Response<GetCategoriesResponse>, Status> {
    let manager_id = CATEGORIES_MANAGE_ID;

    let manage_id = &request.get_ref().manage_id;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(manager_id).unwrap();

    let query_doc = doc! {
        CATEGORIES_MANAGE_ID_FIELD_ID.to_string():manage_id,
    };

    let result = manager.get_query_cursor(query_doc, None, None).await;

    match result {
        Ok(mut entities_iter) => {
            let mut results = vec![];
            while let Some(r) = entities_iter.next().await {
                let d = if let Ok(d) = r {
                    d
                } else {
                    error!("{}-{}", t!("获取失败"), manage_id);
                    continue;
                };

                results.push(bson::to_vec(&d).unwrap());
            }

            Ok(Response::new(GetCategoriesResponse {
                categories: results,
            }))
        }
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}

