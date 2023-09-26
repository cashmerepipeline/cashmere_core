use dependencies_sync::bson::{self, doc, Document};
use dependencies_sync::futures::TryFutureExt;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tokio_stream::{iter, StreamExt};
use dependencies_sync::tonic::async_trait;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::general_field_ids::*;
use managers::traits::ManagerTrait;
use request_utils::request_account_context;

use dependencies_sync::tonic::{Request, Response, Status};
use view::{self, filter_can_read_fields};

use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleCheckEntitiesUpdate {
    /// 取得管理记录数量
    async fn handle_check_entities_update(
        &self,
        request: Request<CheckEntitiesUpdateRequest>,
    ) -> UnaryResponseResult<CheckEntitiesUpdateResponse> {
        validate_view_rules(request)
            .and_then(validate_request_params)
            .and_then(handle_check_entities_update)
            .await
    }
}

async fn validate_view_rules(
    request: Request<CheckEntitiesUpdateRequest>,
) -> Result<Request<CheckEntitiesUpdateRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = &request.get_ref().manage_id;
        let (_account_id, _groups, role_group) = request_account_context(request.metadata());
        if let Err(e) =
            view::validates::validate_collection_can_write(&manage_id, &role_group).await
        {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<CheckEntitiesUpdateRequest>,
) -> Result<Request<CheckEntitiesUpdateRequest>, Status> {
    Ok(request)
}

async fn handle_check_entities_update(
    request: Request<CheckEntitiesUpdateRequest>,
) -> Result<Response<CheckEntitiesUpdateResponse>, Status> {
    let (_account_id, _groups, role_group) = request_account_context(request.metadata());

    let manage_id = &request.get_ref().manage_id;
    let entities = &request.get_ref().entities;

    let majordomo_arc = get_majordomo();
    let manager = majordomo_arc.get_manager_by_id(*manage_id).unwrap();

    let ids = entities
        .iter()
        .map(|e| e.entity_id.clone())
        .collect::<Vec<_>>();

    let query_doc = doc! {
        ID_FIELD_ID.to_string(): {"$in": ids},
    };

    let entity_docs = manager.get_entities_by_filter(&Some(query_doc)).await;

    match entity_docs {
        Ok(r) => {
            let results: Vec<&Document> = r
                .iter()
                .filter(|e| {
                    let id = e.get_str(ID_FIELD_ID.to_string()).unwrap();
                    let m_time = e
                        .get_timestamp(MODIFY_TIMESTAMP_FIELD_ID.to_string())
                        .unwrap();
                    let c_m_time = &entities
                        .iter()
                        .find(|e| e.entity_id == id)
                        .unwrap()
                        .timestamp;

                    // 如果没有设置时间戳，则为需要更新
                    if c_m_time.is_empty() {
                        return true;
                    }

                    m_time
                        > bson::to_bson(c_m_time).unwrap().as_timestamp().unwrap_or(
                            bson::Timestamp {
                                time: 0,
                                increment: 0,
                            },
                        )
                })
                .collect();

            let mut entities = vec![];
            while let Some(e) = iter(&results).next().await {
                let entity = filter_can_read_fields(e, manage_id, &role_group).await;
                entities.push(bson::to_vec(&entity).unwrap());
            }

            Ok(Response::new(CheckEntitiesUpdateResponse {
                entities: entities,
            }))
        }
        Err(e) => {
            Err(Status::data_loss(format!(
                "{}-{}",
                t!("检查实体是否更新错误。"),
                e.details()
            )))
        }
    }
}
