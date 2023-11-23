use dependencies_sync::bson::doc;
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::async_trait;
use dependencies_sync::tonic::{Request, Response, Status};
use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::manager_trait::ManagerTrait;
use managers::utils::make_new_entity_document;
use request_utils::request_account_context;
use validates::validate_name;

#[async_trait]
pub trait HandleAddMember {
    /// 新建管理属性
    async fn handle_add_member(
        &self,
        request: Request<AddMemberRequest>,
    ) -> Result<Response<AddMemberResponse>, Status> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_add_member)
            .await
    }
}

async fn validate_view_rules(
    request: Request<AddMemberRequest>,
) -> Result<Request<AddMemberRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = LANGUAGES_CODES_MANAGE_ID;
        let (account_id, groups, role_group) = request_account_context(request.metadata())?;
        if let Err(e) =
            view::validates::validate_collection_can_write(&manage_id, &role_group).await
        {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<AddMemberRequest>,
) -> Result<Request<AddMemberRequest>, Status> {
    let name = &request.get_ref().name;

    validate_name(name)?;

    Ok(request)
}

async fn handle_add_member(
    request: Request<AddMemberRequest>,
) -> Result<Response<AddMemberResponse>, Status> {
    let (account_id, _groups, role_group) = request_account_context(request.metadata())?;

    let name = &request.get_ref().name;
    let owner_manage_id = &request.get_ref().owner_manage_id;
    let owner_entity_id = &request.get_ref().owner_entity_id;
    let self_manage_id = &request.get_ref().self_manage_id;
    let self_entity_id = &request.get_ref().self_entity_id;

    let manage_id = &MEMBERS_MANAGE_ID;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc
        .get_manager_by_id(manage_id.to_owned())
        .unwrap();

    let name = name.to_owned().unwrap();
    let name_doc = doc! {name.language.clone():name.name.clone()};

    let mut query_doc = doc! {};
    query_doc.insert(
        MEMBERS_OWNER_MANAGE_ID_FIELD_ID.to_string(),
        owner_manage_id.to_owned(),
    );
    query_doc.insert(
        MEMBERS_OWNER_ENTITY_ID_FIELD_ID.to_string(),
        owner_entity_id.to_owned(),
    );
    query_doc.insert(
        MEMBERS_SELF_MANAGE_ID_FIELD_ID.to_string(),
        self_manage_id.to_owned(),
    );
    query_doc.insert(
        MEMBERS_SELF_ENTITY_ID_FIELD_ID.to_string(),
        self_entity_id.to_owned(),
    );

    if let Some(id) = manager.entity_exists(&query_doc).await {
        //标记为删除，则恢复
        if manager.is_mark_removed(&id).await {
            if let Err(err) = manager.recover_removed_entity(&id, &account_id).await {
                return Err(Status::aborted(format!(
                    "{}: {}",
                    t!("恢复删除标记失败"),
                    err.details()
                )));
            };
            return Ok(Response::new(AddMemberResponse { result: id }));
        }
        return Err(Status::already_exists(format!(
            "{}: {}-{}, {}-{}",
            t!("成员已存在"),
            owner_manage_id,
            owner_entity_id,
            self_manage_id,
            self_entity_id
        )));
    }

    if let Some(mut new_entity_doc) = make_new_entity_document(&manager, &account_id).await {
        new_entity_doc.insert(NAME_MAP_FIELD_ID.to_string(), name_doc);
        new_entity_doc.insert(
            MEMBERS_OWNER_MANAGE_ID_FIELD_ID.to_string(),
            owner_manage_id.to_owned(),
        );
        new_entity_doc.insert(
            MEMBERS_OWNER_ENTITY_ID_FIELD_ID.to_string(),
            owner_entity_id.to_owned(),
        );
        new_entity_doc.insert(
            MEMBERS_SELF_MANAGE_ID_FIELD_ID.to_string(),
            self_manage_id.to_owned(),
        );
        new_entity_doc.insert(
            MEMBERS_SELF_ENTITY_ID_FIELD_ID.to_string(),
            self_entity_id.to_owned(),
        );

        let result = manager
            .sink_entity(&mut new_entity_doc, &account_id, &role_group)
            .await;

        match result {
            Ok(r) => Ok(Response::new(AddMemberResponse { result: r })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    } else {
        Err(Status::aborted(format!(
            "{}: {}",
            t!("获取新实体失败"),
            "new_language_code"
        )))
    }
}
