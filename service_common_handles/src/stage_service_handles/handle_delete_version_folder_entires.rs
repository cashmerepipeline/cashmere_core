use dependencies_sync::tonic::async_trait;
use dependencies_sync::bson;
use dependencies_sync::tonic::{Request, Response, Status};

use data_server::data_stage::{delete_version_folder_entries, get_version_folder};
use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;

use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use request_utils::request_account_context;
use view;

use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleDeleteVersionFolderEntries {
    async fn handle_delete_version_folder_entries(
        &self,
        request: Request<DeleteVersionFolderEntriesRequest>,
    ) -> UnaryResponseResult<DeleteVersionFolderEntriesResponse> {
        let (account_id, _groups, role_group) = request_account_context(request.metadata());

        let stage_id = &request.get_ref().stage_id;
        let version = &request.get_ref().version;
        let file_pathes = &request.get_ref().file_pathes;

        // 请求有效性验证
        if stage_id.is_empty() {
            return Err(Status::invalid_argument("stage_id 不能为空"));
        }
        if version.is_empty() {
            return Err(Status::invalid_argument("version 不能为空"));
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
        let stage_manager = majordomo_arc
            .get_manager_by_id(STAGES_MANAGE_ID)
            .unwrap();
        let specses_manager = majordomo_arc
            .get_manager_by_id(SPECSES_MANAGE_ID)
            .unwrap();
        let datas_manager = majordomo_arc
            .get_manager_by_id(DATAS_MANAGE_ID)
            .unwrap();

        let stage_entity = match stage_manager.get_entity_by_id(stage_id).await {
            Ok(r) => r,
            Err(_e) => {
                return Err(Status::not_found(format!(
                    "{}: {}",
                    t!("未找到数据阶段"),
                    stage_id
                )));
            }
        };

        let v = stage_entity
            .get_array(STAGES_VERSIONS_FIELD_ID.to_string())
            .unwrap()
            .iter()
            .map(|v| bson::from_bson::<Version>(v.clone()).unwrap())
            .find(|v| v.name == *version);

        let version_pathes = if let Some(r) = v {
            r.files
        } else {
            return Err(Status::not_found(format!(
                "{}: {}",
                t!("未找到版本"),
                version
            )));
        };

        let tobe_deleted_pathes = file_pathes
            .iter()
            .filter(|p| !version_pathes.contains(p))
            .cloned()
            .collect::<Vec<String>>();
        if tobe_deleted_pathes.is_empty() {
            return Err(Status::not_found(format!(
                "{}: {} {}",
                t!("没有要删除的文件"),
                t!("文件被使用中，无法删除"),
                file_pathes.join(" ")
            )));
        }

        // 将路径字符串换为路径，即替换分隔符
        let tobe_deleted_pathes = tobe_deleted_pathes
            .iter()
            .map(|p| p.replace(',', "/"))
            .collect::<Vec<String>>();

        let specs_id = stage_entity
            .get_str(STAGES_SPECS_ID_FIELD_ID.to_string())
            .unwrap()
            .to_string();
        let specs_entity = match specses_manager.get_entity_by_id(&specs_id).await {
            Ok(r) => r,
            Err(_e) => {
                return Err(Status::not_found(format!(
                    "{}: {}",
                    t!("未找到规格"),
                    specs_id
                )));
            }
        };
        let data_id = specs_entity
            .get_str(SPECSES_DATA_ID_FIELD_ID.to_string())
            .unwrap()
            .to_string();
        let _data_entity = match datas_manager.get_entity_by_id(&data_id).await {
            Ok(r) => r,
            Err(_e) => {
                return Err(Status::not_found(format!(
                    "{}: {}",
                    t!("未找到数据"),
                    data_id
                )));
            }
        };

        let version_foler = match get_version_folder(&data_id, &specs_id, stage_id, version) {
            Ok(r) => r,
            Err(e) => {
                return Err(Status::not_found(format!(
                    "{}: {}",
                    e.operation(),
                    e.details()
                )));
            }
        };

        let result = delete_version_folder_entries(&version_foler, &tobe_deleted_pathes).await;

        match result {
            Ok(_r) => Ok(Response::new(DeleteVersionFolderEntriesResponse {
                result: tobe_deleted_pathes,
            })),
            Err(e) => Err(Status::not_found(format!(
                "{}: {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}
