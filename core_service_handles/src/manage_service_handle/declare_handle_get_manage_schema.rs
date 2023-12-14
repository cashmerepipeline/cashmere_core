/*
宏的形式实现服务组件
*/

#[macro_export]
macro_rules! declare_handle_get_manage_schema {
    ($server:ty) => {
        impl $server {
            /// 取得管理描写
            pub(crate) async fn handle_get_manage_schema(
                &self,
                request: Request<GetManageSchemaRequest>,
            ) -> Result<Response<GetManageSchemaResponse>, Status> {
                let metadata = request.metadata();
                // 已检查过，不需要再检查正确性
                let token = auth::get_auth_token(metadata).unwrap();
                let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

                let manage_id: i32 = request.get_ref().manage_id.parse().unwrap();

                let majordomo_arc = get_majordomo();
                let manager = majordomo_arc.get_manager_by_id(manage_id.as_str()).unwrap();
                let data = manager.get_manage_schema_bytes().await;

                // let data = majordomo_arc.get_manage_schema_bytes(manage_id).await;

                match data {
                    Ok(r) => Ok(Response::new(GetManageSchemaResponse { schema: r })),
                    Err(e) => Err(Status::data_loss(format!(
                        "取得管理描写失败 {} {} ",
                        e.operation(),
                        e.details()
                    ))),
                }
            }
        }
    };
}
