use async_trait::async_trait;

use tonic::{Request, Response, Status};
use bson::doc;

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use request_utils::request_account_context;
use view;

use crate::UnaryResponseResult;

#[async_trait]
pub trait HandleAddFileSetToVersion {
    async fn handle_add_file_set_to_version(
        &self,
        request: Request<AddFileSetToVersionRequest>,
    ) -> UnaryResponseResult<AddFileSetToVersionResponse> {
        let (account_id, _groups, role_group) =
            request_account_context(&request.metadata());

        let stage_id = &request.get_ref().stage_id;
        let version = &request.get_ref().version;
        let file_pathes = &request.get_ref().file_pathes;

        if !view::can_manage_write(&account_id, &role_group, &STAGES_MANAGE_ID.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        if !view::can_field_write(
            &account_id,
            &role_group,
            &STAGES_MANAGE_ID.to_string(),
            &STAGES_VERSIONS_FIELD_ID.to_string(),
        )
        .await
        {
            return Err(Status::permission_denied("用户不具有属性可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(STAGES_MANAGE_ID)
            .await
            .unwrap();

        let query_doc = doc! {
            ID_FIELD_ID.to_string():stage_id,
            format!("{}.name", STAGES_VERSIONS_FIELD_ID):version,
        };

        // let files_bson:Vec<bson::Bson> = file_pathes.iter().map(|p| bson::to_bson(p).unwrap()).collect();

        let field_key = format!("{}.$.files", STAGES_VERSIONS_FIELD_ID);
        let mut modify_doc = bson::Document::new();
        modify_doc.insert(field_key, doc!{"$each": file_pathes});

        let result = manager
            .add_to_array_field(query_doc, modify_doc, &account_id)
            .await;

        match result {
            Ok(_r) => Ok(Response::new(AddFileSetToVersionResponse {
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
