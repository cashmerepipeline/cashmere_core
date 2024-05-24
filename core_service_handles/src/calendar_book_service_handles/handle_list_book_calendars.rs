use dependencies_sync::bson::{self, doc};
use dependencies_sync::futures::TryFutureExt;

use dependencies_sync::tonic::async_trait;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;

use manage_define::manage_ids::{CALENDARS_MANAGE_ID, CALENDAR_BOOKS_MANAGE_ID};
use managers::{entity_interface::EntityInterface};

use dependencies_sync::tonic::{Request, Response, Status};

use service_utils::types::UnaryResponseResult;
use validates::validate_entity_id;

#[async_trait]
pub trait HandleListBookCalendars {
    /// 取得管理记录数量
    async fn handle_list_book_calendars(
        &self,
        request: Request<ListBookCalendarsRequest>,
    ) -> UnaryResponseResult<ListBookCalendarsResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_list_book_calendars)
            .await
    }
}

async fn validate_view_rules(
    request: Request<ListBookCalendarsRequest>,
) -> Result<Request<ListBookCalendarsRequest>, Status> {
    // 全部可读
    Ok(request)
}

async fn validate_request_params(
    request: Request<ListBookCalendarsRequest>,
) -> Result<Request<ListBookCalendarsRequest>, Status> {
    // 没有参数
    let book_id = &request.get_ref().book_id;

    validate_entity_id(CALENDAR_BOOKS_MANAGE_ID, book_id).await?;

    Ok(request)
}

async fn handle_list_book_calendars(
    request: Request<ListBookCalendarsRequest>,
) -> Result<Response<ListBookCalendarsResponse>, Status> {
    let manager_id = CALENDARS_MANAGE_ID;

    let book_id = &request.get_ref().book_id;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(manager_id).unwrap();

    let query_doc = doc! {
        CALENDARS_BOOK_ID_FIELD_ID.to_string():book_id,
    };

    let result = manager.get_entities_by_filter(&Some(query_doc)).await;

    match result {
        Ok(entities) => {
            let r = entities.iter().map(|e| bson::to_vec(e).unwrap()).collect();

            Ok(Response::new(ListBookCalendarsResponse { calendars: r }))
        }
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}
