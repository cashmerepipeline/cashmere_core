use async_trait::async_trait;
use bson::{doc, Document};
use linked_hash_map::LinkedHashMap;
use manage_define::general_field_ids::NAME_MAP_FIELD_ID;
use managers::utils::make_new_entity_document;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;

use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use property_field::{FieldDataType, PropertyField};
use view;

#[async_trait]
pub trait HandleNewGroup {
    /// 新建管理属性
    async fn handle_new_group(
        &self,
        request: Request<NewGroupRequest>,
    ) -> Result<Response<NewGroupResponse>, Status> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let name = &request.get_ref().name;

        let manage_id = &GROUPS_MANAGE_ID;

        if !view::can_manage_write(&account_id, &role_group, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        // 取得第一个可写组作为组
        let account_group_id =
            match view::get_first_write_group(&groups, &manage_id.to_string()).await {
                Some(r) => r,
                None => return Err(Status::unauthenticated("用户不具有可写权限")),
            };

        let majordomo_arc = get_majordomo().await;
        let group_manager = majordomo_arc
            .get_manager_by_id(manage_id.to_owned())
            .await
            .unwrap();

        if let Some(mut new_entity_doc) = make_new_entity_document(&group_manager).await {
            new_entity_doc.insert(
                NAME_MAP_FIELD_ID.to_string(),
                bson::to_document(name).unwrap(),
            );

            let mut group_id = None;
            let result = group_manager
                .sink_entity(&mut new_entity_doc, &account_id, &account_group_id)
                .await
                .and_then(|id| {
                    group_id = Some(id.clone());
                    Ok(())
                });

            match result {
                Ok(_r) => Ok(Response::new(NewGroupResponse {
                    result: group_id.unwrap().clone(),
                })),
                Err(e) => Err(Status::aborted(format!(
                    "{} {}",
                    e.operation(),
                    e.details()
                ))),
            }
        } else {
            Err(Status::aborted("新增数据失败。"))
        }
    }
}
