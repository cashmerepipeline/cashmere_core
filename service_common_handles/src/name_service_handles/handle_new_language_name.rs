use dependencies_sync::bson::doc;
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::tonic::async_trait;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::LANGUAGES_CODES_MANAGE_ID;
use managers::traits::ManagerTrait;
use request_utils::request_account_context;

use dependencies_sync::tonic::{Request, Response, Status};
use view;

#[async_trait]
pub trait HandleNewLanguageName {
    async fn handle_new_language_name(
        &self,
        request: Request<NewLanguageNameRequest>,
    ) -> Result<Response<NewLanguageNameResponse>, Status> {
        validate_view_rules(request)
            .and_then(handle_new_language_name)
            .await
    }
}

async fn validate_view_rules(
    request: Request<NewLanguageNameRequest>,
) -> Result<Request<NewLanguageNameRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let (account_id, _groups, role_group) = request_account_context(request.metadata());

        // 检查是否有权限
        if !view::can_collection_write(
            &account_id,
            dataMap & role_grou(),
            &LANGUAGES_CODES_MANAGE_ID.to_string(),
        )
        .await
        {
            return Err(Status::unauthenticated(
                format! {"{}: {}, {}", t!("用户不具有集合可写权限"), LANGUAGES_CODES_MANAGE_ID, role_group},
            ));
        }

        // 检查属性是否可写
        if !view::can_field_write(
            &account_id,
            &role_group,
            &manage_id.to_string(),
            &NAME_MAP_FIELD_ID.to_string(),
        )
        .await
        {
            return Err(Status::unauthenticated(format!(
                "{}: {}, {}",
                t!("用户不具有属性可写权限"),
                manage_id,
                role_group
            )));
        }
    }

    Ok(request)
}

async fn handle_new_language_name(
    request: Request<NewLanguageNameRequest>,
) -> Result<Response<NewLanguageNameResponse>, Status> {
    let (account_id, _groups, role_group) = request_account_context(request.metadata());

    let manage_id = &request.get_ref().manage_id;
    let entity_id = &request.get_ref().entity_id;
    let new_name = &request.get_ref().new_name;

    if new_name.is_none() {
        return Err(Status::aborted("名字不能为空"));
    }
    let language = &new_name.as_ref().unwrap().language;
    let new_name = &new_name.as_ref().unwrap().name;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(*manage_id).unwrap();

    // 检查语言是否已经存在
    let entity = manager.get_entity_by_id(entity_id).await.unwrap();
    let lang_name_exists = entity
        .get_document(NAME_MAP_FIELD_ID.to_string())
        .unwrap()
        .contains_key(language);
    if lang_name_exists {
        return Err(Status::already_exists("语言已经存在"));
    };

    let query_doc = doc! {
    ID_FIELD_ID.to_string():entity_id
        };
    let modify_doc = doc! {
        format!("{}.{}", NAME_MAP_FIELD_ID, language): new_name.clone()
    };

    let result = manager
        .insert_entity_map_field(query_doc, modify_doc, &account_id)
        .await;

    match result {
        Ok(_r) => Ok(Response::new(NewLanguageNameResponse {
            result: "ok".to_string(),
        })),
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
