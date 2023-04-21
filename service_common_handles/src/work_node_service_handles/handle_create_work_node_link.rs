use async_trait::async_trait;
use bson::doc;
use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use request_utils::request_account_context;
use tonic::{Request, Response, Status};
use view;

use crate::UnaryResponseResult;

#[async_trait]
pub trait HandleCreateWorkNodeLink {
    async fn handle_create_work_node_link(
        &self,
        request: Request<CreateWorkNodeLinkRequest>,
    ) -> UnaryResponseResult<CreateWorkNodeLinkResponse> {
        let (account_id, _groups, role_group) =
            request_account_context(&request.metadata());

        let phase_id = &request.get_ref().phase_id;
        let start_node_id = &request.get_ref().start_node_id;
        let out_slot = &request.get_ref().out_slot;
        let end_node_id = &request.get_ref().end_node_id;
        let in_slot = &request.get_ref().in_slot;

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
        let mut modify_doc = bson::Document::new();
        modify_doc.insert(WORK_NODES_LINKS_FIELD_ID.to_string(), link);

        let result = manager
            .add_to_array_field(query_doc, modify_doc, &account_id)
            .await;

        match result {
            Ok(r) => Ok(Response::new(CreateWorkNodeLinkResponse {
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
