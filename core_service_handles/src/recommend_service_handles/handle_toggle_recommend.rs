use dependencies_sync::{
    bson::{self, doc},
    futures::TryFutureExt,
    rust_i18n::{self, t},
    tonic::async_trait,
    tonic::{Request, Response, Status},
};
use majordomo::{self, get_majordomo};
use manage_define::{
    general_field_ids::{DESCRIPTION_FIELD_ID, NAME_MAP_FIELD_ID, TAGS_FIELD_ID},
    language_keys::CHINESE,
    manage_ids::*,
    field_ids::*,
    cashmere::*,
};
use managers::{utils::make_new_entity_document, ManagerTrait};
use request_utils::request_account_context;
use service_utils::types::UnaryResponseResult;
use validates::{validate_entity_id, validate_name};


#[async_trait]
pub trait HandleToggleRecommend {
    /// 新建产品
    async fn handle_toggle_recommend(
        &self,
        request: Request<ToggleRecommendRequest>,
    ) -> UnaryResponseResult<ToggleRecommendResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_toggle_recommend)
            .await
    }
}

async fn validate_view_rules(
    request: Request<ToggleRecommendRequest>,
) -> Result<Request<ToggleRecommendRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = PRODUCTS_MANAGE_ID;
        let (account_id, _groups, role_group) = request_account_context(request.metadata())?;

        view::validates::validate_collection_can_write(&manage_id, &role_group).await?;
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<ToggleRecommendRequest>,
) -> Result<Request<ToggleRecommendRequest>, Status> {
    let manage_id = &request.get_ref().manage_id;
    let entity_id = &request.get_ref().entity_id;

    validate_entity_id(&manage_id, entity_id).await?;

    Ok(request)
}

async fn handle_toggle_recommend(
    request: Request<ToggleRecommendRequest>,
) -> UnaryResponseResult<ToggleRecommendResponse> {
    let (account_id, _groups, role_group) = request_account_context(request.metadata())?;

    let manage_id = RECOMMENDS_MANAGE_ID;

    let target_manage_id = &request.get_ref().manage_id;
    let entity_id = &request.get_ref().entity_id;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(&manage_id).unwrap();

    // 确保实体存在
    let mut query_doc = doc! {
        RECOMMENDS_MANAGE_ID_FIELD_ID.to_string(): target_manage_id.clone(),
        RECOMMENDS_ENTITY_ID_FIELD_ID.to_string(): entity_id.clone(),
    };

    if manager.entity_exists(&query_doc).await.is_none() {
        // 新建实体
        let mut new_entity_doc =
            if let Ok(doc) = make_new_entity_document(&manager, &account_id).await {
                doc
            } else {
                return Err(Status::internal(t!("新建实体失败").to_string()));
            };

        new_entity_doc.insert(NAME_MAP_FIELD_ID.to_string(), doc! {CHINESE:"默认名"});
        new_entity_doc.insert(
            RECOMMENDS_MANAGE_ID_FIELD_ID.to_string(),
            target_manage_id.clone(),
        );
        new_entity_doc.insert(RECOMMENDS_ENTITY_ID_FIELD_ID.to_string(), entity_id.clone());

        if let Err(err) = manager
            .sink_entity(&mut new_entity_doc, &account_id, &role_group)
            .await
        {
            return Err(Status::internal(err.details()));
        };
    };

    let map_field = format!("{}.{}", RECOMMENDS_RECOMMENDS_MAP_FIELD_ID, account_id);

    let result = if let Ok(entity) = manager.query_entity_map_field(&query_doc, &map_field).await {
        let modify_doc = doc! {
            map_field.clone():{"$set": true},
        };
        // 存在则删除字段
        if let Err(r) = manager
            .delete_entity_map_field_key(query_doc, modify_doc, &account_id)
            .await
        {
            Err(r)
        } else {
            // 没有推荐
            Ok(false)
        }
    } else {
        // 不存在则添加字段
        let modify_doc = doc! {
            map_field:{"$set": true},
        };

        if let Err(err) = manager
            .update_entity_map_field(query_doc, modify_doc, &account_id)
            .await
        {
            Err(err)
        } else {
            // 推荐
            Ok(true)
        }
    };

    match result {
        Ok(_r) => Ok(Response::new(ToggleRecommendResponse { result: _r })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
