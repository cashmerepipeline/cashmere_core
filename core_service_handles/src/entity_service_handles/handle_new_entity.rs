use dependencies_sync::bson::{self, doc, Document};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::log::error;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::async_trait;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::{LANGUAGE_CODES_CODE_FIELD_ID, LANGUAGE_CODES_NATIVE_FIELD_ID};
use manage_define::general_field_ids::{ID_FIELD_ID, NAME_MAP_FIELD_ID};
use manage_define::manage_ids::*;
use managers::entity_interface::EntityInterface;
use managers::utils::make_new_entity_document;
use managers::ManagerInterface;
use request_utils::request_account_context;

use dependencies_sync::tonic::{Request, Response, Status};

#[async_trait]
pub trait HandleNewEntity {
    /// 新建管理属性
    async fn handle_new_entity(
        &self,
        request: Request<NewEntityRequest>,
    ) -> Result<Response<NewEntityResponse>, Status> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_new_entity)
            .await
    }
}

async fn validate_view_rules(
    request: Request<NewEntityRequest>,
) -> Result<Request<NewEntityRequest>, Status> {
    // 只在debug模式下可用
    if !cfg!(debug_assertions) {
        return Err(Status::unavailable(t!("该接口只能在debug模式下使用")));
    }

    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = &request.get_ref().manage_id;
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
    request: Request<NewEntityRequest>,
) -> Result<Request<NewEntityRequest>, Status> {
    let manage_id = &request.get_ref().manage_id;

    // 管理编号不能为0
    if manage_id.is_empty() {
        return Err(Status::invalid_argument(format!(
            "{}-{}",
            t!("管理编号不能为空"),
            "get_entities"
        )));
    }

    Ok(request)
}

async fn handle_new_entity(
    request: Request<NewEntityRequest>,
) -> Result<Response<NewEntityResponse>, Status> {
    let (account_id, _groups, role_group) = request_account_context(request.metadata())?;

    let manage_id = &request.get_ref().manage_id;
    let data = &request.get_ref().data;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(manage_id).unwrap();

    let schema = manager.get_manage_schema().await;
    // schema 不能为空
    if schema.is_empty() {
        return Err(Status::invalid_argument(format!(
            "{}: {}",
            t!("获取管理属性失败"),
            manage_id
        )));
    }

    let new_doc: Document = if let Ok(r) = bson::from_slice(data) {
        r
    } else {
        if cfg!(debug_assertions) {
            error!("{}: {}", t!("无效数据"), manage_id);
        }

        return Err(Status::invalid_argument(format!(
            "{}: {}",
            t!("无效数据"),
            manage_id,
        )));
    };

    if let Ok(mut new_entity_doc) = make_new_entity_document(manager, &account_id).await {
        for (k, v) in new_doc.iter() {
            new_entity_doc.insert(k.to_string(), v.clone());
        }

        let result = manager
            .sink_entity(&mut new_entity_doc, &account_id, &role_group)
            .await;

        match result {
            Ok(r) => Ok(Response::new(NewEntityResponse { result: r })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    } else {
        Err(Status::aborted(format!(
            "{}: {}",
            t!("新建实体失败"),
            "new_entity_code"
        )))
    }
}
