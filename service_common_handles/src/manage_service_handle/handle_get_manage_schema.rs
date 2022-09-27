use async_trait::async_trait;
use bson::doc;
use property_field::PropertyField;
use tokio_stream::StreamExt;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use managers::traits::ManagerTrait;
use view::can_field_read;

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
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let manage_id: i32 = request.get_ref().manage_id.parse().unwrap();

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc.get_manager_by_id(manage_id).await.unwrap();

        let fields = manager.get_manage_schema().await;

        let mut field_stream = tokio_stream::iter(&fields);

        // 可见性过滤
        let mut result: Vec<PropertyField> = vec![];
        while let Some(field) = field_stream.next().await {
            if can_field_read(
                &account_id,
                &role_group,
                &manage_id.to_string(),
                &field.id.to_string(),
            )
            .await
            {
                result.push(field.to_owned());
            }
        }

        // 如果为空则返回空表，无异常发生
        Ok(Response::new(GetManageSchemaResponse {
            fields: result
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
