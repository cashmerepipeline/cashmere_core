use async_trait::async_trait;
use bson::{doc, Document};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;

use managers::traits::ManagerTrait;
use managers::utils::make_new_entity_document;
use crate::name_utils::validate_name;
use crate::UnaryResponseResult;
use tonic::{Request, Response, Status};

#[async_trait]
pub trait HandleNewSpecs{
    /// 新建产品
    async fn handle_new_specs(
        &self,
        request: Request<NewSpecsRequest>,
    ) -> UnaryResponseResult<NewSpecsResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, _groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let name = &request.get_ref().name;
        let data_id = &request.get_ref().data_id;
        let description = &request.get_ref().description;

        if !view::can_collection_write(&account_id, &role_group, &SPECSES_MANAGE_ID.to_string())
            .await
        {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        if validate_name(name).is_err() {
            return Err(Status::data_loss("名字不能为空."));
        }
        let name = name.as_ref().unwrap();

        let majordomo_arc = get_majordomo().await;
        let specs_manager = majordomo_arc
            .get_manager_by_id(SPECSES_MANAGE_ID)
            .await
            .unwrap();

        // 新建条目
        let new_id = specs_manager.get_new_entity_id().await.unwrap();
        let mut new_entity_doc = Document::new();
        new_entity_doc.insert(ID_FIELD_ID.to_string(), new_id.to_string());
        new_entity_doc.insert(
            NAME_MAP_FIELD_ID.to_string(),
            doc! {name.language.clone():name.name.clone()},
        );
        new_entity_doc.insert(
            SPECSES_DATA_ID_FIELD_ID.to_string(),
            data_id.clone(),
        );
        new_entity_doc.insert(DESCRIPTIONS_FIELD_ID.to_string(), description.clone());

        // TODO: 新基础预制件
        let prefabs_manager = majordomo_arc
            .get_manager_by_id(PREFABS_MANAGE_ID)
            .await
            .unwrap();
        let mut basic_prefab_entity_doc = make_new_entity_document(&prefabs_manager).await.unwrap();
        basic_prefab_entity_doc
            .insert(NAME_MAP_FIELD_ID.to_string(), doc! {name.language.clone(): "basic"});
        basic_prefab_entity_doc.insert(PREFABS_SPECS_ID_FIELD_ID.to_string(), new_id.to_string());
        basic_prefab_entity_doc.insert(DESCRIPTIONS_FIELD_ID.to_string(), "basic prefab");

        let new_specs_result = specs_manager
            .sink_entity(&mut new_entity_doc, &account_id, &role_group);
        let new_prefab_result = prefabs_manager
            .sink_entity(&mut basic_prefab_entity_doc, &account_id, &role_group);

        match tokio::try_join!(new_specs_result, new_prefab_result) {
            Ok(r) => Ok(Response::new(NewSpecsResponse {
                // TODO: 发送新建事件
                // 返回specs id
                result: r.0,
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}
