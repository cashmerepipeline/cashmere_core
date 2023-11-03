use dependencies_sync::bson::{self, doc};
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
use service_utils::validate_name;

#[async_trait]
pub trait HandleNewCalendarBook {
    /// 新建管理属性
    async fn handle_new_calendar_book(
        &self,
        request: Request<NewCalendarBookRequest>,
    ) -> Result<Response<NewCalendarBookResponse>, Status> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_new_calendar_book)
            .await
    }
}

async fn validate_view_rules(
    request: Request<NewCalendarBookRequest>,
) -> Result<Request<NewCalendarBookRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = LANGUAGES_CODES_MANAGE_ID;
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
    request: Request<NewCalendarBookRequest>,
) -> Result<Request<NewCalendarBookRequest>, Status> {
    let name = &request.get_ref().name;

    if !validate_name(name) {
        return Err(Status::invalid_argument(format!(
            "{}-{}",
            t!("名字不能为空"),
            "add_member"
        )));
    }

    Ok(request)
}

async fn handle_new_calendar_book(
    request: Request<NewCalendarBookRequest>,
) -> Result<Response<NewCalendarBookResponse>, Status> {
    let (account_id, _groups, role_group) = request_account_context(request.metadata());

    let manage_id = &CALENDAR_BOOKS_MANAGE_ID;

    let name = &request.get_ref().name;
    let param_manage_id = &request.get_ref().manage_id;
    let entity_id = &request.get_ref().entity_id;
    let description = &request.get_ref().description;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc
        .get_manager_by_id(manage_id.to_owned())
        .unwrap();

    let name = name.to_owned().unwrap();
    let name_doc = doc! {name.language.clone():name.name.clone()};

    // zh: 重名检查
    let query_doc = doc! {
        NAME_MAP_FIELD_ID.to_string(): name_doc,
        CALENDAR_BOOKS_MANAGE_ID_FIELD_ID.to_string(): param_manage_id.to_owned(),
        CALENDAR_BOOKS_ENTITY_ID_FIELD_ID.to_string(): entity_id.to_owned(),
    };
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
            return Ok(Response::new(NewCalendarBookResponse { result: id }));
        }
        return Err(Status::already_exists(format!(
            "{}: {}-{}, {:?}",
            t!("成员已存在"),
            param_manage_id,
            entity_id,
            name,
        )));
    }

    if let Some(mut new_entity_doc) = make_new_entity_document(&manager, &account_id).await {
        new_entity_doc.insert(
            NAME_MAP_FIELD_ID.to_string(),
            bson::to_document(&name).unwrap(),
        );
        new_entity_doc.insert(
            CALENDAR_BOOKS_MANAGE_ID_FIELD_ID.to_string(),
            param_manage_id.to_owned(),
        );
        new_entity_doc.insert(CALENDAR_BOOKS_ENTITY_ID_FIELD_ID.to_string(), entity_id.to_owned());
        new_entity_doc.insert(DESCRIPTIONS_FIELD_ID.to_string(), description.to_owned());

        let result = manager
            .sink_entity(&mut new_entity_doc, &account_id, &role_group)
            .await;

        match result {
            Ok(r) => Ok(Response::new(NewCalendarBookResponse { result: r })),
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
