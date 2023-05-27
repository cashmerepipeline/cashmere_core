use dependencies_sync::tonic::async_trait;
use dependencies_sync::bson::{self, doc};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use managers::traits::ManagerTrait;
use request_utils::request_account_context;

use dependencies_sync::tokio_stream::{self as stream, StreamExt};
use dependencies_sync::tonic::{Request, Response, Status};
use view::{self, can_field_read};

use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleGetEntity {
    /// 取得管理记录数量
    async fn handle_get_entity(
        &self,
        request: Request<GetEntityRequest>,
    ) -> UnaryResponseResult<GetEntityResponse> {
        let (account_id, _groups, role_group) =
            request_account_context(request.metadata());

        let manage_id = &request.get_ref().manage_id;
        let entity_id = &request.get_ref().entity_id;

        let majordomo_arc = get_majordomo();
        let manager = majordomo_arc.get_manager_by_id(*manage_id).unwrap();

        let result = manager.get_entity_by_id(entity_id).await;

        match result {
            Ok(r) => {
                // 字段可见性过滤
                let mut result_doc = doc!();
                let mut property_stream = stream::iter(r);
                while let Some((k, v)) = property_stream.next().await {
                    if !can_field_read(&account_id, &role_group, &manage_id.to_string(), &k).await {
                        if k == *"_id" {
                            result_doc.insert(k, v);
                        }
                        continue;
                    }
                    result_doc.insert(k, v);
                }

                Ok(Response::new(GetEntityResponse {
                    entity: bson::to_vec(&result_doc).unwrap(),
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
