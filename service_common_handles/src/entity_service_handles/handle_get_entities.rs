use dependencies_sync::bson::{self, doc};
use dependencies_sync::tonic::async_trait;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::general_field_ids::ID_FIELD_ID;
use managers::traits::ManagerTrait;
use request_utils::request_account_context;

use dependencies_sync::tokio_stream::{self as stream, StreamExt};
use dependencies_sync::tonic::{Request, Response, Status};
use view::{self, can_entity_read, can_field_read};

use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleGetEntities {
    /// 取得管理记录数量
    async fn handle_get_entities(
        &self,
        request: Request<GetEntitiesRequest>,
    ) -> UnaryResponseResult<GetEntitiesResponse> {
        let (account_id, _groups, role_group) = request_account_context(request.metadata());

        let manage_id = &request.get_ref().manage_id;
        let entity_ids = &request.get_ref().entity_ids;

        let majordomo_arc = get_majordomo();
        let manager = majordomo_arc.get_manager_by_id(*manage_id).unwrap();

        // 实体可见性过滤
        let mut filtered_ids = vec![];
        let mut id_stream = stream::iter(entity_ids);
        while let Some(ref id) = id_stream.next().await {
            if can_entity_read(&manage_id.to_string(), &role_group).await {
                filtered_ids.push(id.to_owned());
            }
        }

        let query_doc = doc! {
            ID_FIELD_ID.to_string():{"$in":filtered_ids.clone()}
        };

        let result = manager.get_entities_by_filter(&Some(query_doc)).await;

        match result {
            Ok(entities) => {
                // 格可见性过滤
                let mut result_docs = vec![];

                let entity_iter = entities.iter();
                for e in entity_iter {
                    let mut result_doc = doc!();
                    let mut property_stream = stream::iter(e);

                    while let Some((k, v)) = property_stream.next().await {
                        if !can_field_read(  &manage_id.to_string(), k, &role_group)
                            .await
                        {
                            if k == &"_id".to_string() {
                                result_doc.insert(k, v);
                            }
                            continue;
                        }
                        result_doc.insert(k, v);
                    }

                    result_docs.push(result_doc);
                }

                Ok(Response::new(GetEntitiesResponse {
                    entities: result_docs
                        .iter()
                        .map(|x| bson::to_vec(x).unwrap())
                        .collect(),
                }))
            }
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}


async fn validate_view_rules(
    request: Request<GetEntitiesRequest>,
) -> Result<Request<GetEntitiesRequest>, Status> {
    #[cfg(feature = "view_rules_validate")]
    {
        let manage_id = &request.get_ref().manage_id;
        let (_account_id, _groups, role_group) = request_account_context(request.metadata());
        if let Err(e) = view::validates::validate_collection_can_write(&manage_id, &role_group).await {
            return Err(e);
        }
    }

    Ok(request)
}

async fn validate_request_params(
    request: Request<GetEntitiesRequest>,
) -> Result<Request<GetEntitiesRequest>, Status> {
    Ok(request)
}