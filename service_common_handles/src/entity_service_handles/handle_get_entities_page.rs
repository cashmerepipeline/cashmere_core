use async_trait::async_trait;
use bson::doc;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use managers::traits::ManagerTrait;
use view;

use crate::UnaryResponseResult;

#[async_trait]
pub trait HandleGetEntitiesPage {
    /// 取得产品分页
    async fn handle_get_entities_page(
        &self,
        request: Request<GetEntitiesPageRequest>,
    ) -> UnaryResponseResult<GetEntitiesPageResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

        let manage_id = &request.get_ref().manage_id;
        let page_index = &request.get_ref().page_index;
        let conditions = &request.get_ref().conditions;

        // TODO: 可读性检查

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc.get_manager_by_id(*manage_id).await.unwrap();

        let conditions_doc = bson::to_document(conditions).unwrap();

        let result = manager
            .get_entities_by_page(*page_index, &None, &Some(conditions_doc))
            .await;

        // TODO: 可见性过滤

        match result {
            Ok(entities) => Ok(Response::new(GetEntitiesPageResponse {
                entities: entities
                    .iter()
                    .map(|x| bson::from_document(x.clone()).unwrap())
                    .collect(),
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}
