/*
宏的形式实现服务组件
*/

#[macro_export]
macro_rules! declare_handle_new_schema_field {
    ($server:ty) => {
        impl $server {
            /// 新建管理属性
            pub(crate) async fn handle_new_schema_field(
                &self,
                request: Request<NewSchemaFieldRequest>,
            ) -> Result<Response<NewSchemaFieldResponse>, Status> {
                let metadata = request.metadata();
                // 已检查过，不需要再检查正确性
                let token = auth::get_auth_token(metadata).unwrap();
                let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

                if !view::can_manage_write(&account_id, &groups, &MANAGES_MANAGE_ID.to_string())
                    .await
                {
                    return Err(Status::unauthenticated("用户不具有可写权限"));
                }

                let manage_id: i32 = request.get_ref().manage_id.parse().unwrap();
                let field: &Field = request.get_ref().field.as_ref().unwrap();

                let mut name_bytes = field.name.clone();
                let name_doc = Document::from_reader(&mut name_bytes.as_slice()).unwrap();
                let name: LinkedHashMap<String, String> = bson::from_document(name_doc).unwrap();

                let new_field: PropertyField = PropertyField {
                    id: field.id,
                    name: name,
                    data_type: FieldDataType::from(field.data_type.clone()),
                    removed: false,
                };

                let majordomo_arc = get_majordomo().await;
                let manager = majordomo_arc.get_manager_by_id(manage_id).await.unwrap();
                let result = manager.new_schema_field(new_field, &account_id).await;

                match result {
                    Ok(r) => Ok(Response::new(NewSchemaFieldResponse {
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

