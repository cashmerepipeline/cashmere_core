use async_trait::async_trait;
use bson::doc;
use manage_define::cashmere::*;
use manage_define::manage_ids::DATAS_MANAGE_ID;
use request_utils::request_account_context;

use tonic::{Request, Response, Status};

#[async_trait]
pub trait HandleGetDataServerConfigs {
    /// 取得管理
    async fn handle_get_data_server_configs(
        &self,
        request: Request<GetDataServerConfigsRequest>,
    ) -> Result<Response<GetDataServerConfigsResponse>, Status> {
        let (account_id, _groups, role_group) =
            request_account_context(request.metadata());

        //TODO: 权限检查可能需要其他管理项
        if !view::can_manage_read(&account_id, &role_group, &DATAS_MANAGE_ID.to_string()).await {
            return Err(Status::data_loss("没有管理读取权限"));
        }

        let data_server_configs = bson::to_document(&configs::get_data_server_configs()).unwrap();

        Ok(Response::new(GetDataServerConfigsResponse {
            configs: bson::from_document(data_server_configs).unwrap(),
        }))
    }
}
