use dependencies_sync::tonic::async_trait;
use dependencies_sync::bson::{self, doc};

use manage_define::cashmere::*;
use manage_define::manage_ids::DATAS_MANAGE_ID;
use request_utils::request_account_context;

use dependencies_sync::tonic::{Request, Response, Status};

#[async_trait]
pub trait HandleGetDataServerConfigs {
    /// 取得管理
    async fn handle_get_data_server_configs(
        &self,
        request: Request<GetDataServerConfigsRequest>,
    ) -> Result<Response<GetDataServerConfigsResponse>, Status> {
        let (account_id, _groups, role_group) =
            request_account_context(request.metadata());

        let data_server_configs = bson::to_document(&configs::get_data_server_configs()).unwrap();

        Ok(Response::new(GetDataServerConfigsResponse {
            configs: bson::from_document(data_server_configs).unwrap(),
        }))
    }
}
