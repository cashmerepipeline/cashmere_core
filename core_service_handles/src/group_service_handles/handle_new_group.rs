use dependencies_sync::bson::doc;
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::async_trait;
use dependencies_sync::log::info;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::general_field_ids::{ID_FIELD_ID, NAME_MAP_FIELD_ID};
use manage_define::manage_ids::*;
use managers::manager_trait::ManagerTrait;
use managers::utils::make_new_entity_document;
use request_utils::request_account_context;

use dependencies_sync::tonic::{Request, Response, Status};

#[async_trait]
pub trait HandleNewGroup {
    /// 新建管理属性
    async fn handle_new_group(
        &self,
        request: Request<NewGroupRequest>,
    ) -> Result<Response<NewGroupResponse>, Status> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_new_group)
            .await
    }
}

async fn validate_view_rules(
    request: Request<NewGroupRequest>,
) -> Result<Request<NewGroupRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = GROUPS_MANAGE_ID;
        let (account_id, groups, role_group) = request_account_context(request.metadata());
        if let Err(e) =
            view::validates::validate_collection_can_write(&manage_id, &role_group).await
        {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<NewGroupRequest>,
) -> Result<Request<NewGroupRequest>, Status> {
    Ok(request)
}

async fn handle_new_group(
    request: Request<NewGroupRequest>,
) -> Result<Response<NewGroupResponse>, Status> {
    let (account_id, _groups, role_group) = request_account_context(request.metadata());

    let name = &request.get_ref().name;
    let new_group_id = &request.get_ref().new_group_id;

    let manage_id = &GROUPS_MANAGE_ID;

    let majordomo_arc = get_majordomo();
    let group_manager = majordomo_arc
        .get_manager_by_id(manage_id.to_owned())
        .unwrap();

    //TODO: 组编号是否符合格式

    // 组是否已经存在
    if group_manager
        .entity_exists(&doc! {ID_FIELD_ID.to_string():new_group_id})
        .await
    {
        return Err(Status::already_exists(format!(
            "{}: {}",
            t!("组已经存在"),
            new_group_id
        )));
    }

    let name = match name {
        Some(n) => n,
        None => {
            return Err(Status::aborted(format!("没有指定名称--{}", new_group_id)));
        }
    };

    if let Some(mut new_entity_doc) = make_new_entity_document(&group_manager, &account_id).await {
        new_entity_doc.insert(
            NAME_MAP_FIELD_ID.to_string(),
            doc! {name.language.clone():name.name.clone()},
        );
        new_entity_doc.insert("_id".to_string(), new_group_id);
        new_entity_doc.insert(ID_FIELD_ID.to_string(), new_group_id);

        info!("{}: {}", t!("开始创建新组"), new_group_id);
        let result = group_manager
            .sink_entity(&mut new_entity_doc, &account_id, &role_group)
            .await;

        match result {
            Ok(r) => Ok(Response::new(NewGroupResponse { result: r })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    } else {
        Err(Status::aborted(format!("{}: {}", t!("获取新实体失败"), "new_group")))
    }
}
