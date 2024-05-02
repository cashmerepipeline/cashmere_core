use dependencies_sync::{
    bson::{doc},
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
    manage_ids::*,
};
use managers::{ManagerTrait};
use request_utils::request_account_context;
use service_utils::types::UnaryResponseResult;
use validates::{validate_manage_id};

#[async_trait]
pub trait HandleGetAccountRecommendedEntities {
    /// 新建产品
    async fn handle_get_account_recommended_entities(
        &self,
        request: Request<GetAccountRecommendedEntitiesRequest>,
    ) -> UnaryResponseResult<GetAccountRecommendedEntitiesResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_get_account_recommended_entities)
            .await
    }
}

async fn validate_view_rules(
    request: Request<GetAccountRecommendedEntitiesRequest>,
) -> Result<Request<GetAccountRecommendedEntitiesRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = PRODUCTS_MANAGE_ID;
        let (account_id, _groups, role_group) = request_account_context(request.metadata())?;

        view::validates::validate_collection_can_write(&manage_id, &role_group).await?;
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<GetAccountRecommendedEntitiesRequest>,
) -> Result<Request<GetAccountRecommendedEntitiesRequest>, Status> {
    let manage_id = &request.get_ref().manage_id;

    validate_manage_id(manage_id).await?;

    Ok(request)
}

async fn handle_get_account_recommended_entities(
    request: Request<GetAccountRecommendedEntitiesRequest>,
) -> UnaryResponseResult<GetAccountRecommendedEntitiesResponse> {
    let (account_id, _groups, _role_group) = request_account_context(request.metadata())?;

    let manage_id = RECOMMENDS_MANAGE_ID;

    let target_manage_id = &request.get_ref().manage_id;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(manage_id).unwrap();

    // 确保实体存在
    let query_doc = doc! {
        RECOMMENDS_MANAGE_ID_FIELD_ID.to_string(): target_manage_id.clone(),
        RECOMMENDS_ACCOUNT_FIELD_ID.to_string(): account_id.clone(),
    };

    let mut stream = match manager
        .get_entity_stream(
            query_doc,
            &[],
            None,
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
    let mut getted_count = 0u32;
    while let Some(entity) = stream.next().await {
        if getted_count > 1000 {
            break;
        }

        result.push(
            entity
                .get_str(RECOMMENDS_ENTITY_ID_FIELD_ID.to_string())
                .unwrap()
                .to_string(),
        );

        getted_count += 1;
    }

    Ok(Response::new(GetAccountRecommendedEntitiesResponse {
        entities: result,
    }))
}
