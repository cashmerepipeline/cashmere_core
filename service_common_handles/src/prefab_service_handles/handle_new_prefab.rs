use async_trait::async_trait;
use bson::{doc, Document};
use dependencies_sync::tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use request_utils::request_account_context;
use service_utils::validate_name;
use crate::UnaryResponseResult;

#[async_trait]
pub trait HandleNewPrefab{
    /// 新建产品
    async fn handle_new_prefab(
        &self,
        request: Request<NewPrefabRequest>,
    ) -> UnaryResponseResult<NewPrefabResponse> {
        let (account_id, _groups, role_group ) = request_account_context(request.metadata());

        let name = &request.get_ref().name;
        let specs_id = &request.get_ref().specs_id;
        let _stage_id = &request.get_ref().stage_id;
        let _modifies = &request.get_ref().modifies;
        let description = &request.get_ref().description;

        if !view::can_collection_write(&account_id, &role_group, &PREFABS_MANAGE_ID.to_string())
            .await
        {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        if validate_name(name).is_err() {
            return Err(Status::data_loss("名字不能为空."));
        }
        let name = name.as_ref().unwrap();

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(PREFABS_MANAGE_ID)
            .await
            .unwrap();

        // 新建条目
        let new_id = manager.get_new_entity_id().await.unwrap();
        let mut new_entity_doc = Document::new();
        new_entity_doc.insert(ID_FIELD_ID.to_string(), new_id.to_string());
        new_entity_doc.insert(
            NAME_MAP_FIELD_ID.to_string(),
            doc! {name.language.clone():name.name.clone()},
        );
        new_entity_doc.insert(
            PREFABS_SPECS_ID_FIELD_ID.to_string(),
            specs_id.clone()
        );
        new_entity_doc.insert(
            DESCRIPTIONS_FIELD_ID.to_string(),
            description.clone()
        );
        
        let result = manager
            .sink_entity(&mut new_entity_doc, &account_id, &role_group)
            .await;

        match result {
            Ok(r) => Ok(Response::new(NewPrefabResponse {
                // TODO: 发送新建事件
                result: r,
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}


