/*
Author: 闫刚 (yes7rose@sina.com)
point_handle_service.rs (c) 2021
Desc: 通用服务
Created:  2021-01-23T23:52:52.002Z
Modified: !date!
*/

#[macro_export]
macro_rules! declare_handle_rename {
    ($server:ty) => {
        impl $server {
            pub(crate) async fn handle_rename(
                &self,
                request: Request<RenameRequest>,
            ) -> Result<Response<RenameResponse>, Status> {
                let metadata = request.metadata();
                // 已检查过，不需要再检查正确性
                let token = auth::get_auth_token(metadata).unwrap();
                let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

                let manage_id = &request.get_ref().manage_id;
                let entity_id = &request.get_ref().entity_id;
                let language = &request.get_ref().language;
                let name = &request.get_ref().new_name;

                if !view::can_manage_write(&account_id, &groups, manage_id).await {
                    return Err(Status::unauthenticated("用户不具有可写权限"));
                }

                let majordomo_arc = get_majordomo().await;
                let manager = majordomo_arc
                    .get_manager_by_id(manage_id.parse::<i32>().unwrap())
                    .await
                    .unwrap();

                let query_doc = doc! {
                    "_id":entity_id
                };
                let modify_doc = doc! {
                    format!("{}.{}", NAME_FIELD_ID, language):name.clone()
                };

                let result = manager
                    .update_entity_map_field(query_doc, modify_doc, &account_id)
                    .await;

                match result {
                    Ok(r) => Ok(Response::new(RenameResponse {
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

#[macro_export]
macro_rules! declare_handle_new_language_name {
    ($server:ty) => {
        impl $server {
            pub async fn handle_new_language_name(
                &self,
                request: Request<NewLanguageNameRequest>,
            ) -> Result<Response<NewLanguageNameResponse>, Status> {
                let metadata = request.metadata();
                // 已检查过，不需要再检查正确性
                let token = auth::get_auth_token(metadata).unwrap();
                let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

                let manage_id = &request.get_ref().manage_id;
                let entity_id = &request.get_ref().entity_id;
                let language = &request.get_ref().language;
                let new_name = &request.get_ref().new_name;

                if !view::can_manage_write(&account_id, &groups, manage_id).await {
                    return Err(Status::unauthenticated("用户不具有可写权限"));
                }

                let majordomo_arc = get_majordomo().await;
                let manager = majordomo_arc
                    .get_manager_by_id(manage_id.parse::<i32>().unwrap())
                    .await
                    .unwrap();

                let query_doc = doc! {
                    "_id":entity_id
                };
                let modify_doc = doc! {
                    format!("{}", NAME_FIELD_ID):{language.clone():new_name.clone()}
                };

                let result = manager
                    .insert_entity_map_field(query_doc, modify_doc, &account_id)
                    .await;

                match result {
                    Ok(r) => Ok(Response::new(NewLanguageNameResponse {
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
