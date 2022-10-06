use async_trait::async_trait;
use bson::doc;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use managers::utils::make_new_entity_document;
use view;

use crate::UnaryResponseResult;

#[async_trait]
pub trait HandleNewArea {
    async fn handle_new_area(
        &self,
        request: Request<NewAreaRequest>,
    ) -> UnaryResponseResult<NewAreaResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let parent_id = &request.get_ref().parent_id;
        let name = &request.get_ref().name;
        let level = &request.get_ref().level;
        let code = &request.get_ref().code;

        if !view::can_manage_write(&account_id, &role_group, &AREAS_MANAGE_ID.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }
        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(AREAS_MANAGE_ID)
            .await
            .unwrap();

        let local_name =match name {
            Some(n)=>n,
            None => {
                return Err(Status::aborted(format!("没有指定名称--{}", code)));
            }
        };

        let name_doc = doc!{local_name.language.clone():local_name.name.clone()};

        // 区域是否存在，存在则返回
        if manager
            .entity_exists(doc! {
                NAME_MAP_FIELD_ID.to_string():name_doc.clone(),
            })
            .await
        {
            return Err(Status::aborted("区域已经存在"));
        }

        if let Some(mut new_entity_doc) = make_new_entity_document(&manager).await {
            new_entity_doc.insert("_id", code);
            new_entity_doc.insert(ID_FIELD_ID.to_string(), code);
            new_entity_doc.insert(NAME_MAP_FIELD_ID.to_string(), name_doc);
            new_entity_doc.insert(AREAS_PARENT_ID_FIELD_ID.to_string(), parent_id);
            new_entity_doc.insert(AREAS_LEVEL_FIELD_ID.to_string(), level);

            let result = manager
                .sink_entity(&mut new_entity_doc, &account_id, &role_group)
                .await;

            match result {
                Ok(_r) => Ok(Response::new(NewAreaResponse {
                    result: "ok".to_string(),
                })),
                Err(e) => Err(Status::aborted(format!(
                    "{} {}",
                    e.operation(),
                    e.details()
                ))),
            }
        } else {
            Err(Status::aborted("创建新区域失败"))
        }
    }
}

