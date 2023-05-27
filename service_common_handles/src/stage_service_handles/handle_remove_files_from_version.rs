use dependencies_sync::tonic::async_trait;
use dependencies_sync::bson::{self, doc};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use dependencies_sync::tonic::{Request, Response, Status};
use request_utils::request_account_context;
use view;

use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleRemoveFilesFromVersion {
    async fn handle_remove_files_from_version(
        &self,
        request: Request<RemoveFilesFromVersionRequest>,
    ) -> UnaryResponseResult<RemoveFilesFromVersionResponse> {
        let (account_id, _groups, role_group) = request_account_context(request.metadata());

        let stage_id = &request.get_ref().stage_id;
        let version = &request.get_ref().version;
        let file_pathes = &request.get_ref().file_pathes;
        
        // 输入检查
        if stage_id.is_empty() {
            return Err(Status::invalid_argument("阶段ID为空"));
        }
        if version.is_empty() {
            return Err(Status::invalid_argument("版本名为空"));
        }
        if file_pathes.is_empty() {
            return Err(Status::invalid_argument("文件路径列表为空"));
        }

        let majordomo_arc = get_majordomo();
        let manager = majordomo_arc
            .get_manager_by_id(STAGES_MANAGE_ID)
            .unwrap();

        let query_doc = doc! {
            ID_FIELD_ID.to_string():stage_id,
            format!("{}.name", STAGES_VERSIONS_FIELD_ID):version,
        };

        let field_key = format!("{}.$.files", STAGES_VERSIONS_FIELD_ID);
        let mut modify_doc = bson::Document::new();
        modify_doc.insert(field_key, doc!("$each":file_pathes));

        let result = manager
            .remove_from_array_field(query_doc, modify_doc, &account_id)
            .await;

        match result {
            Ok(_r) => Ok(Response::new(RemoveFilesFromVersionResponse {
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

