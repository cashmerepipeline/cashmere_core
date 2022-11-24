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
pub trait HandleNewDataSlotForWorkNode{
    async fn handle_new_data_slot_for_work_node(
        &self,
        request: Request<NewDataSlotForWorkNodeRequest>,
    ) -> UnaryResponseResult<NewDataSlotForWorkNodeResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let node_id = &request.get_ref().node_id;
        let slot_name = &request.get_ref().slot_name;
        let slot_type = &request.get_ref().slot_type;

        let slot_type = SlotType::from(slot_type).unwrap();

        if !view::can_entity_write(
            &account_id,
            &role_group,
            &WORK_NODES_MANAGE_ID.to_string(),
        )
        .await
        {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(WORK_NODES_MANAGE_ID)
            .await
            .unwrap();

        let result = match slot_type {
            SlotType::RefrenceData => {
                // TODO: need fill
                let query_doc = doc! {
                    
                };
                let modify_doc = doc!{};
                manager
                    .insert_entity_map_field(
                        query_doc,
                        modify_doc,
                        &account_id,
                    )
                    .await
            }
            SlotType::DepedentData => {
                // TODO: need fill
                let query_doc = doc! {
                    
                };
                let modify_doc = doc!{};
                manager
                    .insert_entity_map_field(
                        query_doc,
                        modify_doc,
                        &account_id,
                    )
                    .await
            }
            SlotType::WorkData => {
                // TODO: need fill
                let query_doc = doc! {
                    
                };
                let modify_doc = doc!{};
                manager
                    .insert_entity_map_field(
                        query_doc,
                        modify_doc,
                        &account_id,
                    )
                    .await
            }
            SlotType::OutData => {
                // TODO: need fill
                let query_doc = doc! {
                    
                };
                let modify_doc = doc!{};
                manager
                    .insert_entity_map_field(
                        query_doc,
                        modify_doc,
                        &account_id,
                    )
                    .await
            }
        };

        match result {
            Ok(r) => Ok(Response::new(NewDataSlotForWorkNodeResponse {
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

