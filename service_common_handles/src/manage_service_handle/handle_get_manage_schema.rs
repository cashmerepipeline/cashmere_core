use async_trait::async_trait;
use bson::doc;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use managers::traits::ManagerTrait;

#[async_trait]
pub trait HandleGetManageSchema {
    /// 取得管理描写
    async fn handle_get_manage_schema(
        &self,
        request: Request<GetManageSchemaRequest>,
    ) -> Result<Response<GetManageSchemaResponse>, Status> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (_account_id, _groups) = auth::get_claims_account_and_roles(&token).unwrap();

        let manage_id: i32 = request.get_ref().manage_id.parse().unwrap();

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc.get_manager_by_id(manage_id).await.unwrap();

        let fields = manager.get_manage_schema().await;

        // TODO: 可见性过滤

        // let fields = majordomo_arc.get_manage_schema_bytes(manage_id).await;

        // 如果为空则返回空表，无异常发生
        Ok(Response::new(GetManageSchemaResponse {
            fields: fields
                .iter()
                .map(|f| {
                    let rf = SchemaField {
                        id: f.id,
                        name_map: bson::to_vec(&f.name_map).unwrap(),
                        data_type: f.data_type.to_string(),
                        removed: f.removed,
                    };

                    rf
                })
                .collect(),
        }))
    }
}
