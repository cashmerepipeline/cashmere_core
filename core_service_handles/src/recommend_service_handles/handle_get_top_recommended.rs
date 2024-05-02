use dependencies_sync::{
    bson::{self, doc},
    futures::TryFutureExt,
    log::{error},
    rust_i18n::{self, t},
    tokio_stream::StreamExt,
    tonic::{async_trait, Request, Response, Status},
};

use manage_define::{
    cashmere::*,
    manage_ids::*,
};

use request_utils::request_account_context;
use service_utils::types::UnaryResponseResult;
use validates::{validate_manage_id};

use super::query_top_recommends;

#[async_trait]
pub trait HandleGetTopRecommends {
    /// 新建产品
    async fn handle_get_top_recommends(
        &self,
        request: Request<GetTopRecommendsRequest>,
    ) -> UnaryResponseResult<GetTopRecommendsResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_get_top_recommends)
            .await
    }
}

async fn validate_view_rules(
    request: Request<GetTopRecommendsRequest>,
) -> Result<Request<GetTopRecommendsRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = PRODUCTS_MANAGE_ID;
        let (account_id, _groups, role_group) = request_account_context(request.metadata())?;

        view::validates::validate_collection_can_write(&manage_id, &role_group).await?;
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<GetTopRecommendsRequest>,
) -> Result<Request<GetTopRecommendsRequest>, Status> {
    let manage_id = &request.get_ref().manage_id;
    let count = &request.get_ref().count;

    validate_manage_id(manage_id.as_str()).await?;

    if !(&1..=&1000).contains(&count) {
        return Err(Status::invalid_argument(format!(
            "{} {} {} {}",
            t!("推荐数量不能超过"),
            1000,
            t!("不能少于"),
            1
        )));
    }

    Ok(request)
}

async fn handle_get_top_recommends(
    request: Request<GetTopRecommendsRequest>,
) -> UnaryResponseResult<GetTopRecommendsResponse> {
    let (_account_id, _groups, _role_group) = request_account_context(request.metadata())?;

    let _manage_id = RECOMMENDS_MANAGE_ID;

    let target_manage_id = &request.get_ref().manage_id;
    let count = &request.get_ref().count;

    let result = query_top_recommends::query_top_recommends(target_manage_id, count).await;

    match result {
        Ok(r) => {
            let bs = r
                .iter()
                .map(|x| bson::to_vec(x).unwrap())
                .collect::<Vec<_>>();
            Ok(Response::new(GetTopRecommendsResponse { recommends: bs }))
        }
        Err(e) => {
            error!("{}: {}", t!("查询推荐失败"), e.details());

            Err(Status::internal(format!(
                "{}: {}",
                t!("查询推荐失败"),
                target_manage_id
            )))
        }
    }
}
