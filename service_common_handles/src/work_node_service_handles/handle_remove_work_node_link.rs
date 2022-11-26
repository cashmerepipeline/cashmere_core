use async_trait::async_trait;
use bson::doc;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use manage_define::field_ids::*;
use managers::traits::ManagerTrait;
use view;

use crate::UnaryResponseResult;

#[async_trait]
pub trait HandleRemoveWorkNodeLink {
    async fn handle_remove_work_node_link(
        &self,
        request: Request<RemoveWorkNodeLinkRequest>,
    ) -> UnaryResponseResult<RemoveWorkNodeLinkResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let phase_id = &request.get_ref().phase_id;
        let start_node_id = &request.get_ref().start_node_id;
        let out_slot = &request.get_ref().out_slot;
        let end_node_id = &request.get_ref().end_node_id;
        let in_slot = &request.get_ref().in_slot;

        let end_node_id = &request.get_ref().end_node_id;

        if !view::can_entity_write(&account_id, &role_group, &WORK_NODES_MANAGE_ID.to_string())
            .await
        {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(WORK_NODES_MANAGE_ID)
            .await
            .unwrap();

        let mut link = bson::Document::new();
        link.insert("start_node", start_node_id);
        link.insert("out_slot", start_node_id);
        link.insert("end_node", start_node_id);
        link.insert("in_slot", start_node_id);

        let query_doc = doc! {
            ID_FIELD_ID.to_string():phase_id,
        };
        // TODO: fill doc
        let mut modify_doc = bson::Document::new();
        modify_doc.insert(WORK_NODES_LINKS_FIELD_ID.to_string(), link);

        let result = manager
            .pull_entity_array_field(query_doc, modify_doc, &account_id)
            .await;

        match result {
            Ok(r) => Ok(Response::new(RemoveWorkNodeLinkResponse {
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
