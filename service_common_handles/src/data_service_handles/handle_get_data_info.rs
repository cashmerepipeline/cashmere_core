use async_trait::async_trait;
use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use request_utils::request_account_context;

use tonic::{Request, Response, Status};
use view;

use crate::UnaryResponseResult;

#[async_trait]
pub trait HandleGetDataInfo {
    async fn handle_get_data_info(
        &self,
        request: Request<GetDataInfoRequest>,
    ) -> UnaryResponseResult<GetDataInfoResponse> {
        let (account_id, _groups, role_group) =
            request_account_context(request.metadata());

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
            Ok(r) => Ok(Response::new(GetDataInfoResponse {
                data_info: Some(DataInfo {
                    data_type: r.get_i32(DATAS_DATA_TYPE_FIELD_ID.to_string()).unwrap(),
                    owner_manage_id: r
                        .get_i32(DATAS_OWNER_MANAGE_ID_FIELD_ID.to_string())
                        .unwrap(),
                    owner_entity_id: r
                        .get_str(DATAS_OWNER_ENTITY_ID_FIELD_ID.to_string())
                        .unwrap()
                        .to_string(),
                    specs: r
                        .get_array(DATAS_SPECS_FIELD_ID.to_string())
                        .unwrap_or(&vec![])
                        .iter()
                        .map(|x| String::from(x.as_str().unwrap()))
                        .collect(),
                }),
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}
