use async_trait::async_trait;
use bson::doc;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::general_field_ids::*;
use managers::traits::ManagerTrait;
use view;

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

        let node_id = &request.get_ref().node_id;
        let slot_name = &request.get_ref().slot_name;
        let slot_type = &request.get_ref().slot_type;

        let slot_type = SlotType::from(slot_type).unwrap();

        if !view::can_entity_write(
            &account_id,
            &groups,
            &WORK_NODES_MANAGE_ID.to_string(),
            &WORK_NODE_REFERENCE_DATAS_FIELD_ID.to_string(),
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
                manager
                    .insert_entity_map_field(
                        node_id,
                        &WORK_NODE_DEPENDED_DATAS_FIELD_ID.to_string(),
                        slot_name,
                        bson::to_bson("").unwrap(),
                        &account_id,
                    )
                    .await
            }
            SlotType::WorkData => {
                manager
                    .insert_entity_map_field(
                        node_id,
                        &WORK_NODE_WORK_DATAS_FIELD_ID.to_string(),
                        slot_name,
                        bson::to_bson("").unwrap(),
                        &account_id,
                    )
                    .await
            }
            SlotType::OutData => {
                manager
                    .insert_entity_map_field(
                        node_id,
                        &WORK_NODE_OUT_DATAS_FIELD_ID.to_string(),
                        slot_name,
                        bson::to_bson("").unwrap(),
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

