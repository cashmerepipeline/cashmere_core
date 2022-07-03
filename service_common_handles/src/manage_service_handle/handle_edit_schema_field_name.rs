use async_trait::async_trait;
use bson::doc;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use managers::traits::ManagerTrait;
use view;

#[async_trait]
pub trait HandleEditSchemaFieldName {
    ///编辑管理属性字段名
    async fn handle_edit_schema_field_name(
        &self,
        request: Request<EditSchemaFieldNameRequest>,
    ) -> Result<Response<EditSchemaFieldNameResponse>, Status> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

        let manage_id = &request.get_ref().manage_id;
        let field_id = request.get_ref().field_id;
        let language = &request.get_ref().language;
        let new_name = &request.get_ref().new_name;

        // 需要系统管理员权限
        if !view::can_manage_write(&account_id, &groups, manage_id).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(manage_id.parse().unwrap())
            .await
            .unwrap();
        let result = manager
            .edit_schema_field_name(field_id, language, new_name, &account_id)
            .await;

        match result {
            Ok(_r) => Ok(Response::new(EditSchemaFieldNameResponse {
                result: "ok".to_string(),
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}
