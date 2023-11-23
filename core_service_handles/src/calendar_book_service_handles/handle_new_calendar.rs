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
use validates::validate_name;

#[async_trait]
pub trait HandleNewCalendar {
    /// 新建管理属性
    async fn handle_new_calendar(
        &self,
        request: Request<NewCalendarRequest>,
    ) -> Result<Response<NewCalendarResponse>, Status> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_new_calendar)
            .await
    }
}

async fn validate_view_rules(
    request: Request<NewCalendarRequest>,
) -> Result<Request<NewCalendarRequest>, Status> {
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
    request: Request<NewCalendarRequest>,
) -> Result<Request<NewCalendarRequest>, Status> {
    let name = &request.get_ref().name;

    validate_name(name)?;

    Ok(request)
}

async fn handle_new_calendar(
    request: Request<NewCalendarRequest>,
) -> Result<Response<NewCalendarResponse>, Status> {
    let (account_id, _groups, role_group) = request_account_context(request.metadata())?;

    let manage_id = &CALENDARS_MANAGE_ID;

    let name = &request.get_ref().name;
    let book_id = &request.get_ref().book_id;
    let calendar = &request.get_ref().calendar;
    let description = &request.get_ref().description;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc
        .get_manager_by_id(manage_id.to_owned())
        .unwrap();

    let name = name.to_owned().unwrap();
    let name_doc = doc! {name.language.clone():name.name.clone()};

    if let Some(mut new_entity_doc) = make_new_entity_document(&manager, &account_id).await {
        new_entity_doc.insert(
            NAME_MAP_FIELD_ID.to_string(),
            name_doc,
        );
        new_entity_doc.insert(
            CALENDARS_BOOK_ID_FIELD_ID.to_string(),
            book_id.to_owned(),
        );
        new_entity_doc.insert(CALENDARS_CALENDAR_FIELD_ID.to_string(), bson::to_document(calendar).unwrap());
        new_entity_doc.insert(DESCRIPTION_FIELD_ID.to_string(), description.to_owned());

        let result = manager
            .sink_entity(&mut new_entity_doc, &account_id, &role_group)
            .await;

        match result {
            Ok(r) => Ok(Response::new(NewCalendarResponse { result: r })),
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

