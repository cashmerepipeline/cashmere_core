use dependencies_sync::tonic::async_trait;
use dependencies_sync::bson::{self, doc};

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
        let (account_id, _groups, role_group) =
            request_account_context(request.metadata());

        let manage_id = &request.get_ref().manage_id;
        let entity_ids = &request.get_ref().entity_ids;

        // 管理可读性检查
        if !view::can_manage_read(&account_id, &role_group, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有管理可读权限"));
        }

        // 集合可读性检查
        if !view::can_collection_read(&account_id, &role_group, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有集合可读权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc.get_manager_by_id(*manage_id).await.unwrap();

        // 实体可见性过滤
        let mut filtered_ids = vec![];
        let mut id_stream = stream::iter(entity_ids);
        while let Some(ref id) = id_stream.next().await {
            if can_entity_read(&account_id, &role_group, &manage_id.to_string()).await {
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
                        if !can_field_read(&account_id, &role_group, &manage_id.to_string(), k)
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
