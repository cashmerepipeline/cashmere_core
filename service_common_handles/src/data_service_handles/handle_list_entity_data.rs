use async_trait::async_trait;
use bson::doc;
use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::general_field_ids::*;
use managers::traits::ManagerTrait;
use request_utils::request_account_context;

use tonic::{Request, Response, Status};
use view;

use crate::UnaryResponseResult;

#[async_trait]
pub trait HandleListEntityData {
    /// 取得管理记录数量
    async fn handle_list_entity_data(
        &self,
        request: Request<ListEntityDataRequest>,
    ) -> UnaryResponseResult<ListEntityDataResponse> {
        let (account_id, _groups, role_group) =
            request_account_context(&request.metadata());

        let manage_id = &request.get_ref().manage_id;
        let entity_id = &request.get_ref().entity_id;

        if !view::can_manage_read(&account_id, &role_group, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可读权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc.get_manager_by_id(*manage_id).await.unwrap();

        let result = manager.get_entity_by_id(entity_id).await;

        match result {
            Ok(r) => {
                let data_ids = r.get_array(DATAS_FIELD_ID.to_string()).unwrap();
                Ok(Response::new(ListEntityDataResponse {
                    data_ids: data_ids.iter().map(|x| x.to_string()).collect(),
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
