use dependencies_sync::bson::{self, doc};
use dependencies_sync::futures::TryFutureExt;

use dependencies_sync::tonic::async_trait;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;

use manage_define::manage_ids::{TAGS_MANAGE_ID, CALENDAR_BOOKS_MANAGE_ID};
use managers::manager_trait::ManagerTrait;

use dependencies_sync::tokio_stream::StreamExt;
use dependencies_sync::tonic::{Request, Response, Status};

use service_utils::types::UnaryResponseResult;
use validates::{validate_manage_id, validate_entity_id};

#[async_trait]
pub trait HandleListCalendarBooks {
    /// 取得管理记录数量
    async fn handle_list_calendar_books(
        &self,
        request: Request<ListCalendarBooksRequest>,
    ) -> UnaryResponseResult<ListCalendarBooksResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_list_calendar_books)
            .await
    }
}

async fn validate_view_rules(
    request: Request<ListCalendarBooksRequest>,
) -> Result<Request<ListCalendarBooksRequest>, Status> {
    // 全部可读
    Ok(request)
}

async fn validate_request_params(
    request: Request<ListCalendarBooksRequest>,
) -> Result<Request<ListCalendarBooksRequest>, Status> {
    // 没有参数
    let manage_id = &request.get_ref().manage_id;
    let entity_id = &request.get_ref().entity_id;
    
    validate_manage_id(manage_id).await?;
    validate_entity_id(manage_id, entity_id).await?;

    Ok(request)
}

async fn handle_list_calendar_books(
    request: Request<ListCalendarBooksRequest>,
) -> Result<Response<ListCalendarBooksResponse>, Status> {
    let manager_id = CALENDAR_BOOKS_MANAGE_ID;

    let manage_id = &request.get_ref().manage_id;
    let entity_id = &request.get_ref().entity_id;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(manager_id).unwrap();

    let query_doc = doc! {
        CALENDAR_BOOKS_MANAGE_ID_FIELD_ID.to_string():manage_id,
        CALENDAR_BOOKS_ENTITY_ID_FIELD_ID.to_string():entity_id,
    };

    let result  = manager.get_entities_by_filter(&Some(query_doc)).await;

    match result {
        Ok(entities) => {
            let r = entities.iter().map(|e| bson::to_vec(e).unwrap()).collect();

            Ok(Response::new(ListCalendarBooksResponse { books: r }))
        }
        Err(e) => Err(Status::aborted(format!(
            "{} {}",
            e.operation(),
            e.details()
        ))),
    }
}

