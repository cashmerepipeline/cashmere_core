use dependencies_sync::tonic::async_trait;
use dependencies_sync::bson::{self, doc};

use dependencies_sync::tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use request_utils::request_account_context;
use view;

use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleAddFileSequenceToVersion {
    async fn handle_add_file_sequence_to_version(
        &self,
        request: Request<AddFileSequenceToVersionRequest>,
    ) -> UnaryResponseResult<AddFileSequenceToVersionResponse> {
        let (account_id, _groups, role_group) =
            request_account_context(request.metadata());

        let stage_id = &request.get_ref().stage_id;
        let version = &request.get_ref().version;
        let base_name = &request.get_ref().base_name;
        let start = &request.get_ref().start;
        let end = &request.get_ref().end;
        let padding = &request.get_ref().padding;
        let extension = &request.get_ref().extension;

        // 检查参数
        if stage_id.is_empty() {
            return Err(Status::invalid_argument("stage_id 不能为空"));
        }
        if version.is_empty() {
            return Err(Status::invalid_argument("version 不能为空"));
        }
        if base_name.is_empty() {
            return Err(Status::invalid_argument("base_name 不能为空"));
        }
        if start < &0 {
            return Err(Status::invalid_argument("start 不能小于 0"));
        }
        if end < &0 {
            return Err(Status::invalid_argument("end 不能小于 0"));
        }
        if end < start {
            return Err(Status::invalid_argument("end 不能小于 start"));
        }
        if padding < &0 {
            return Err(Status::invalid_argument("padding 不能小于 0"));
        }
        if extension.is_empty() {
            return Err(Status::invalid_argument("extension 不能为空"));
        }

        

        // 权限检查
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

        let majordomo_arc = get_majordomo();
        let manager = majordomo_arc
            .get_manager_by_id(STAGES_MANAGE_ID)
            .unwrap();

        let query_doc = doc! {
            ID_FIELD_ID.to_string():stage_id,
            format!("{}.name", STAGES_VERSIONS_FIELD_ID):version,
        };

        // 创建序列list
        let seq_vec: Vec<String> = vec![
            base_name.to_string(),
            start.to_string(),
            end.to_string(),
            padding.to_string(),
            extension.to_string(),
        ];

        let field_key = format!("{}.$.files", STAGES_VERSIONS_FIELD_ID);
        let mut modify_doc = bson::Document::new();
        modify_doc.insert(field_key, seq_vec);

        let result = manager
            .add_to_array_field(query_doc, modify_doc, &account_id)
            .await;

        match result {
            Ok(_r) => Ok(Response::new(AddFileSequenceToVersionResponse {
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
