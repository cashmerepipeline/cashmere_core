use async_trait::async_trait;
use bson::{doc, Document};
use linked_hash_map::LinkedHashMap;
use log::info;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::general_field_ids::{ID_FIELD_ID, NAME_MAP_FIELD_ID};
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use managers::utils::make_new_entity_document;
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
        let new_group_id = &request.get_ref().new_group_id;

        let manage_id = &GROUPS_MANAGE_ID;

        info!("开始创建新组{}", new_group_id);
        // 检查组可写
        if !view::can_collection_write(&account_id, &role_group, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有组可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let group_manager = majordomo_arc
            .get_manager_by_id(manage_id.to_owned())
            .await
            .unwrap();

        //TODO: 组编号是否符合格式

        // 组是否已经存在
        if group_manager.entity_exists(doc! {ID_FIELD_ID.to_string():new_group_id}).await{
            return Err(Status::already_exists(format!("组已经存在: {}", new_group_id)));
        }

        let name = match name {
            Some(n)=>n,
            None => {
                return Err(Status::aborted(format!("没有指定名称--{}", new_group_id)));
            }
        };

        if let Some(mut new_entity_doc) = make_new_entity_document(&group_manager).await {
            new_entity_doc.insert(
                NAME_MAP_FIELD_ID.to_string(),
                doc! {name.language.clone():name.name.clone()}
            );
            new_entity_doc.insert("_id".to_string(), new_group_id);
            new_entity_doc.insert(ID_FIELD_ID.to_string(), new_group_id);

            info!("开始创建新组{}", new_group_id);
            let result = group_manager
                .sink_entity(&mut new_entity_doc, &account_id, &role_group)
                .await
                .and_then(|id| {
                    Ok(id)
                });

            match result {
                Ok(r) => Ok(Response::new(NewGroupResponse {
                    result: r,
                })),
                Err(e) => Err(Status::aborted(format!(
                    "{} {}",
                    e.operation(),
                    e.details()
                ))),
            }
        } else {
            Err(Status::aborted("新增组失败。"))
        }
    }
}
