use dependencies_sync::bson::{doc};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::rust_i18n::{self, t};

use dependencies_sync::tonic::async_trait;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;

use request_utils::request_account_context;

use dependencies_sync::tonic::{Request, Response, Status};


use search_engine::search;
use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleSearch {
    /// 取得管理记录数量
    async fn handle_search(
        &self,
        request: Request<SearchRequest>,
    ) -> UnaryResponseResult<SearchResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_search)
            .await
    }
}

async fn validate_view_rules(
    request: Request<SearchRequest>,
) -> Result<Request<SearchRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = &request.get_ref().manage_id;
        let (_account_id, _groups, role_group) = request_account_context(request.metadata())?;
        if let Err(e) = view::validates::validate_collection_can_read(&manage_id, &role_group).await
        {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<SearchRequest>,
) -> Result<Request<SearchRequest>, Status> {
    let manage_id = &request.get_ref().manage_id;
    let search_params = &request.get_ref().search_params;
    if search_params.is_empty() {
        return Err(Status::invalid_argument(format!(
            "{} {}",
            t!("搜索参数不能为空"),
            manage_id
        )));
    }
    Ok(request)
}

async fn handle_search(
    request: Request<SearchRequest>,
) -> Result<Response<SearchResponse>, Status> {
    let (_account_id, _groups, _role_group) = request_account_context(request.metadata())?;

    let manage_id = &request.get_ref().manage_id;
    let search_params = &request.get_ref().search_params;

    let search_str = search_params
        .iter()

        // zh: 关键词必须用引号包裹, 否则空格位置会不正确
        .map(|(k, v)| format!("{}:\"{}\"", k, v))
        .collect::<Vec<String>>()
        .join(" ");

    let majordomo_arc = get_majordomo();
    let _manager = majordomo_arc.get_manager_by_id(manage_id.as_str()).unwrap();

    let result = search(manage_id, search_str.as_str()).await;

    // TODO:  过滤不可读实体

    match result {
        Ok(r) => Ok(Response::new(SearchResponse { results: r })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
