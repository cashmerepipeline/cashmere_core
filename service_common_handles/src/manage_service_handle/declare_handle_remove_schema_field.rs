/*
宏的形式实现服务组件
*/

#[macro_export]
macro_rules! declare_handle_remove_schema_field {
    ($server:ty) => {
        impl $server {
            /// 移除管理属性
            pub(crate) async fn handle_remove_schema_field(
                &self,
                request: Request<RemoveSchemaFieldRequest>,
            ) -> Result<Response<RemoveSchemaFieldResponse>, Status> {
                let metadata = request.metadata();
                // 已检查过，不需要再检查正确性
                let token = auth::get_auth_token(metadata).unwrap();
                let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

                let manage_id: i32 = request.get_ref().manage_id.parse().unwrap();
                let field_id = request.get_ref().field_id;

                if !view::can_manage_write(&account_id, &groups, &manage_id.to_string()).await {
                    return Err(Status::unauthenticated("用户不具有可写权限"));
                }

                let majordomo_arc = get_majordomo();
                let manager = majordomo_arc.get_manager_by_id(manage_id).unwrap();
                let result = manager
                    .mark_schema_field_removed(field_id, &account_id)
                    .await;

                match result {
                    Ok(r) => Ok(Response::new(RemoveSchemaFieldResponse {
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
    };
}
