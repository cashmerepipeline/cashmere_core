use async_trait::async_trait;
use bson::doc;

use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::general_field_ids::NAME_FIELD_ID;
use manage_define::manage_ids::MANAGES_MANAGE_ID;
use managers::traits::ManagerTrait;
use view;

#[async_trait]
pub trait HandleGetManages {
    /// 取得管理
    async fn handle_get_manages(
        &self,
        request: Request<GetManagesRequest>,
    ) -> Result<Response<GetManagesResponse>, Status> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

        let managers_ids: Vec<i32> = get_majordomo().await.get_manager_ids().await;

        // TOTO: 可见性过滤

        let mut result: Vec<Manage> = Vec::new();
        for id in managers_ids {
            let manager = get_majordomo().await.get_manager_by_id(id).await.unwrap();
            let doc = manager.get_manage_document().await.read().clone();
            let m = Manage {
                manage_id: doc
                    .get_str(MANAGES_MANAGE_ID.to_string())
                    .unwrap()
                    .to_string(),
                name_map: doc
                    .get_binary_generic(NAME_FIELD_ID.to_string())
                    .unwrap()
                    .to_vec(),
            };
            result.push(m);
        }

        Ok(Response::new(GetManagesResponse { manages: result }))
    }
}
