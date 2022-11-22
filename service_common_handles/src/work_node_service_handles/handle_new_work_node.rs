use async_trait::async_trait;
use bson::doc;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::general_field_ids::*;
use manage_define::field_ids::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use view;

#[async_trait]
pub trait HandleNewWorkNode {
    async fn handle_new_work_node(
        &self,
        request: Request<NewWorkNodeRequest>,
    ) -> Result<Response<NewWorkNodeResponse>, Status> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

        let procedure_id = &request.get_ref().procedure_id;
        let node_name = &request.get_ref().node_name;

        if !view::can_entity_write(
            &account_id,
            &groups,
            &PROCEDURES_MANAGE_ID.to_string(),
            &PROCEDURE_WORK_NODES_FIELD_ID.to_string(),
        )
            .await
        {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        if !view::can_manage_write(&account_id, &groups, &WORK_NODES_MANAGE_ID.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        // 取得第一个可写组作为组
        let group_id =
            match view::get_first_write_group(&groups, &WORK_NODES_MANAGE_ID.to_string()).await {
                Some(r) => r,
                None => return Err(Status::unauthenticated("用户不具有可写权限")),
            };

        let majordomo_arc = get_majordomo().await;
        let node_manager = majordomo_arc
            .get_manager_by_id(WORK_NODES_MANAGE_ID)
            .await
            .unwrap();

        let procedure_mamager = majordomo_arc
            .get_manager_by_id(PROCEDURES_MANAGE_ID)
            .await
            .unwrap();

        let node_id = node_manager.get_new_entity_id().await.unwrap();
        let mut new_doc = Document::new();
        new_doc.insert("_id".to_string(), node_id.to_string());
        new_doc.insert(ID_FIELD_ID.to_string(), node_id.to_string());
        new_doc.insert(NAME_FIELD_ID.to_string(), node_name);
        new_doc.insert(WORK_NODE_PROCEDURE_FIELD_ID.to_string(), procedure_id);

        let result = node_manager
            .new_entity(&mut new_doc, &account_id, &group_id)
            .await;

        match result {
            Ok(r) => {
                // 添加成功后，更新过程
                let query_doc = doc! {
                    "_id":procedure_id
                };
                let modify_doc = doc! {
                     PROCEDURE_WORK_NODES_FIELD_ID.to_string():node_id
                };
                match procedure_mamager
                    .push_entity_array_field(query_doc, modify_doc, &account_id)
                    .await
                {
                    Err(e) => {
                        return Err(Status::aborted(format!(
                            "{} {}",
                            e.operation(),
                            e.details()
                        )));
                    }
                    _ => (),
                }
                Ok(Response::new(NewWorkNodeResponse {
                    result: "ok".to_string(),
                }))
            }
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}
