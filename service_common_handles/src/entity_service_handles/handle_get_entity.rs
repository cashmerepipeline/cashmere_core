use async_trait::async_trait;
use bson::doc;
use tokio_stream::{self as stream, StreamExt};
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use managers::traits::ManagerTrait;
use view::{self, can_field_read};

use crate::UnaryResponseResult;

#[async_trait]
pub trait HandleGetEntity {
    /// 取得管理记录数量
    async fn handle_get_entity(
        &self,
        request: Request<GetEntityRequest>,
    ) -> UnaryResponseResult<GetEntityResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

        let manage_id = &request.get_ref().manage_id;
        let entity_id = &request.get_ref().entity_id;

        // 管理可读性检查
        if !view::can_manage_read(&account_id, &groups, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有管理可读权限"));
        }

        // 集合可读性检查
        if !view::can_collection_read(&account_id, &groups, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有集合可读权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc.get_manager_by_id(*manage_id).await.unwrap();

        let result = manager.get_entity_by_id(entity_id).await;

        // 实体可读性检查
        if !view::can_entity_read(&account_id, &groups, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有实体可读权限"));
        };

        // 格可见性过滤
        let mut result_doc = doc!();
        if let Ok(ref entity_doc) = result {
            while let Some((k, v)) = stream::iter(entity_doc).next().await {
                if can_field_read(&account_id, &groups, &manage_id.to_string(), &k).await {
                    result_doc.insert(k, v);
                }
            }
        };

        match result {
            Ok(_r) => Ok(Response::new(GetEntityResponse {
                entity: bson::from_document(result_doc).unwrap(),
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}
