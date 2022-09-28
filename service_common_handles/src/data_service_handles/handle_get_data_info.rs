

use async_trait::async_trait;




use tonic::{Request, Response, Status};

use manage_define::cashmere::*;
use crate::UnaryResponseResult;

use majordomo::{self, get_majordomo};


use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;

use view;

#[async_trait]
pub trait HandleGetDataInfo {
    async fn handle_get_data_info(
        &self,
        request: Request<GetDataInfoRequest>,
    ) -> UnaryResponseResult<GetDataInfoResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let data_id = &request.get_ref().data_id;

        if !view::can_manage_write(&account_id, &role_group, &DATAS_MANAGE_ID.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let data_manager = majordomo_arc
            .get_manager_by_id(DATAS_MANAGE_ID)
            .await
            .unwrap();

        let result = data_manager.get_entity_by_id(data_id).await;
        match result {
            Ok(r) => {
                let mut bytes: Vec<u8> = Vec::new();
                r.to_writer(&mut bytes).expect(&*format!("数据损坏:{}", data_id));

                Ok(Response::new(GetDataInfoResponse {
                    data: bytes
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