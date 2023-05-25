use dependencies_sync::bson::doc;
use dependencies_sync::tonic::async_trait;

use dependencies_sync::tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;

use service_utils::types::UnaryResponseResult;
use service_utils::validate_name;

use managers::traits::ManagerTrait;
use managers::utils::make_new_entity_document;
use request_utils::request_account_context;

#[async_trait]
pub trait HandleNewSpecs {
    /// 新建产品
    async fn handle_new_specs(
        &self,
        request: Request<NewSpecsRequest>,
    ) -> UnaryResponseResult<NewSpecsResponse> {
        let (account_id, _groups, role_group) = request_account_context(request.metadata());

        let data_id = &request.get_ref().data_id;
        let name = &request.get_ref().name;
        let description = &request.get_ref().description;

        if validate_name(name).is_err() {
            return Err(Status::data_loss(format!(
                "{}: {}",
                t!("名字不能为空"),
                data_id
            )));
        }
        let local_name = match name {
            Some(n) => n,
            None => {
                return Err(Status::aborted(format!("没有指定名称--{}", data_id)));
            }
        };
        let name_doc = doc! {local_name.language.clone():local_name.name.clone()};

        if !view::can_collection_write(&account_id, &role_group, &SPECSES_MANAGE_ID.to_string())
            .await
        {
            return Err(Status::unauthenticated(format!(
                "{}: {}",
                t!("用户不具有可写权限"),
                data_id
            )));
        }

        let majordomo_arc = get_majordomo();
        let specs_manager = majordomo_arc
            .get_manager_by_id(SPECSES_MANAGE_ID)
            .unwrap();

        // 新建条目
        let mut new_entity_doc = if let Some(r) = make_new_entity_document(&specs_manager).await {
            r
        } else {
            return Err(Status::aborted(format!(
                "{}: 管理 {}",
                t!("新建实体文档失败"),
                SPECSES_MANAGE_ID
            )));
        };

        new_entity_doc.insert(SPECSES_DATA_ID_FIELD_ID.to_string(), data_id.clone());
        new_entity_doc.insert(NAME_MAP_FIELD_ID.to_string(), name_doc);
        new_entity_doc.insert(DESCRIPTIONS_FIELD_ID.to_string(), description.clone());

        let new_specs_result = specs_manager
            .sink_entity(&mut new_entity_doc, &account_id, &role_group)
            .await;

        match new_specs_result {
            Ok(r) => Ok(Response::new(NewSpecsResponse { result: r })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}
