use dependencies_sync::{
    bson::{self, doc},
    futures::TryFutureExt,
    rust_i18n::{self, t},
    tokio_stream::StreamExt,
    tonic::async_trait,
    tonic::{Request, Response, Status},
};
use majordomo::{self, get_majordomo};
use manage_define::{
    cashmere::*,
    field_ids::*,
    general_field_ids::{DESCRIPTION_FIELD_ID, NAME_MAP_FIELD_ID, TAGS_FIELD_ID},
    language_keys::CHINESE,
    manage_ids::*,
};
use managers::{utils::make_new_entity_document, ManagerTrait};
use request_utils::request_account_context;
use service_utils::types::UnaryResponseResult;
use validates::{validate_entity_id, validate_name};

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
    let entity_id = &request.get_ref().entity_id;
    let count = &request.get_ref().count;

    validate_entity_id(&manage_id, entity_id).await?;

    if count > &1000 || count < &1 {
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
    let (account_id, _groups, role_group) = request_account_context(request.metadata())?;

    let manage_id = RECOMMENDS_MANAGE_ID;

    let target_manage_id = &request.get_ref().manage_id;
    let entity_id = &request.get_ref().entity_id;
    let count = &request.get_ref().count;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(&manage_id).unwrap();

    // 确保实体存在
    let query_doc = doc! {
        RECOMMENDS_MANAGE_ID_FIELD_ID.to_string(): target_manage_id.clone(),
        RECOMMENDS_ENTITY_ID_FIELD_ID.to_string(): entity_id.clone(),
    };

    let sort_doc = doc! {
    RECOMMENDS_RECOMMENDS_COUNT_FIELD_ID.to_string(): -1,
    };

    let mut stream = match manager
        .get_entity_stream(
            query_doc,
            &[RECOMMENDS_RECOMMENDS_MAP_FIELD_ID.to_string()],
            Some(sort_doc),
            None,
            0,
        )
        .await
    {
        Ok(r) => r,
        Err(e) => {
            return Err(Status::aborted(format!(
                "{} {}",
                t!("取得推荐列表失败"),
                e.details()
            )));
        }
    };

    let mut result: Vec<String> = vec![];
    let mut getted_count = 0;
    while let Some(entity) = stream.next().await {
        if &getted_count > count || getted_count > 1000 {
            break;
        }

        result.push(
            entity
                .get_str(RECOMMENDS_RECOMMENDS_MAP_FIELD_ID.to_string())
                .unwrap()
                .to_string(),
        );

        getted_count += 1;
    }

    Ok(Response::new(GetTopRecommendsResponse {
        recommend_list: result,
    }))
}
