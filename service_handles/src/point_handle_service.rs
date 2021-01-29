/*
Author: 闫刚 (yes7rose@sina.com)
point_handle_service.rs (c) 2021
Desc: 知识点服务
Created:  2021-01-23T23:52:52.002Z
Modified: !date!
*/

use auth::jwt::validate_is_root;
use cash_core::field::ids::*;
use cash_core::field::{FieldDataType, Name, PropertyField};
use cash_core::ids::*;
use majordomo::{self, get_majordomo};
use managers::traits::ManagerTrait;
use view;

use bson::{doc, Document};
use tonic::{Request, Response, Status};

use crate::cashmere::*;

#[macro_export]
macro_rules! declare_handle_new_point {
    ($server:ty) => {
        impl $server {
            pub(crate) async fn handle_new_point(
                &self,
                request: Request<NewPointRequest>,
            ) -> UnaryResponseResult<NewPointResponse> {
                let metadata = request.metadata();
                // 已检查过，不需要再检查正确性
                let token = auth::get_auth_token(metadata).unwrap();
                let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

                let domain = &request.get_ref().domain_id;
                let local = &request.get_ref().local;
                let name = &request.get_ref().name;

                if !view::can_manage_write(&account_id, &groups, &POINTS_MANAGE_ID.to_string())
                    .await
                {
                    return Err(Status::unauthenticated("用户不具有可写权限"));
                }
                // 取得第一个可写组作为组
                let group_id =
                    match view::get_first_write_group(&groups, &POINTS_MANAGE_ID.to_string()).await
                    {
                        Some(r) => r,
                        None => return Err(Status::unauthenticated("用户不具有可写权限")),
                    };

                let mut new_entity_doc = Document::new();

                let majordomo_arc = get_majordomo().await;
                let manager = majordomo_arc
                    .get_manager_by_id(POINTS_MANAGE_ID)
                    .await
                    .unwrap();
                let new_id = manager.get_new_entity_id().await.unwrap();

                let local_name = doc! {
                    local.clone(): name.clone()
                };

                new_entity_doc.insert("_id", new_id.to_string());
                new_entity_doc.insert(ID_FIELD_ID.to_string(), new_id.to_string());
                new_entity_doc.insert(NAME_FIELD_ID.to_string(), local_name);

                let result = manager
                    .new_entity(&mut new_entity_doc, &account_id, &group_id)
                    .await;

                match result {
                    Ok(r) => Ok(Response::new(NewPointResponse {
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
