use dependencies_sync::tonic::async_trait;
use dependencies_sync::bson::{doc};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::general_field_ids::*;
use managers::traits::ManagerTrait;
use request_utils::request_account_context;

use dependencies_sync::tonic::{Request, Response, Status};
use view;

#[async_trait]
pub trait HandleRename {
    async fn handle_rename(
        &self,
        request: Request<RenameRequest>,
    ) -> Result<Response<RenameResponse>, Status> {
        let (account_id, _groups, role_group) = request_account_context(request.metadata());

        let manage_id = &request.get_ref().manage_id;
        let entity_id = &request.get_ref().entity_id;
        let new_name = &request.get_ref().new_name;

        if new_name.is_none() {
            return Err(Status::aborted("名字不能为空"));
        }
        let language = &new_name.as_ref().unwrap().language;
        let new_name = &new_name.as_ref().unwrap().name;

        if !view::can_manage_write(&account_id, &role_group, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        // 集合可写性检查
        if !view::can_collection_read(&account_id, &role_group, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有集合可读权限"));
        }

        // 检查属性是否可写
        if !view::can_field_write(
            &account_id,
            &role_group,
            &manage_id.to_string(),
            &NAME_MAP_FIELD_ID.to_string(),
        )
        .await
        {
            return Err(Status::unauthenticated("用户不具有集合可读权限"));
        }

        let majordomo_arc = get_majordomo();
        let manager = majordomo_arc.get_manager_by_id(*manage_id).unwrap();

        let query_doc = doc! {
            ID_FIELD_ID.to_string():entity_id
        };
        let modify_doc = doc! {
            format!("{}.{}", NAME_MAP_FIELD_ID, language):new_name.clone()
        };

        let result = manager
            .update_entity_map_field(query_doc, modify_doc, &account_id)
            .await;

        match result {
            Ok(_r) => Ok(Response::new(RenameResponse {
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
