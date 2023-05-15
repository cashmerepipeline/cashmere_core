use async_trait::async_trait;
use bson::doc;
use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use managers::traits::ManagerTrait;
use property_field::PropertyField;
use request_utils::request_account_context;

use tokio_stream::StreamExt;
use dependencies_sync::tonic::{Request, Response, Status};
use view::can_field_read;

#[async_trait]
pub trait HandleGetManageSchema {
    /// 取得管理描写
    async fn handle_get_manage_schema(
        &self,
        request: Request<GetManageSchemaRequest>,
    ) -> Result<Response<GetManageSchemaResponse>, Status> {
        let (account_id, _groups, role_group) =
            request_account_context(request.metadata());

        let manage_id = request.get_ref().manage_id;

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
                    

                    SchemaField {
                        id: f.id,
                        name_map: bson::to_vec(&f.name_map).unwrap(),
                        data_type: f.data_type.to_string(),
                        removed: f.removed,
                    }
                })
                .collect(),
        }))
    }
}
