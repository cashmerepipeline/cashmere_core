use async_trait::async_trait;
use bson::doc;
use manage_define::cashmere::*;
use manage_define::manage_ids::DATAS_MANAGE_ID;
use tonic::{Request, Response, Status};

#[async_trait]
pub trait HandleGetDataServerConfigs {
    /// 取得管理
    async fn handle_get_data_server_configs(
        &self,
        request: Request<GetDataServerConfigsRequest>,
    ) -> Result<Response<GetDataServerConfigsResponse>, Status> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, _groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        //TODO: 权限检查可能需要其他管理项
        if !view::can_manage_read(&account_id, &role_group, &DATAS_MANAGE_ID.to_string()).await{
            return Err(Status::data_loss("没有管理读取权限"));
        }

        let data_server_configs = bson::to_document(&configs::get_data_server_configs()).unwrap();

        Ok(Response::new(GetDataServerConfigsResponse { configs: bson::from_document(data_server_configs).unwrap()}))
    }
}
