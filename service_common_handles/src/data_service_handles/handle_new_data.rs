use async_trait::async_trait;
use bson::doc;
use futures::TryFutureExt;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use managers::utils::make_new_entity_document;
use view;

use crate::UnaryResponseResult;

#[async_trait]
pub trait HandleNewData {
    async fn handle_new_data(
        &self,
        request: Request<NewDataRequest>,
    ) -> UnaryResponseResult<NewDataResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let manage_id = &request.get_ref().owner_manage_id;
        let entity_id = &request.get_ref().owner_entity_id;
        let data_type = &request.get_ref().data_type;
        let name = &request.get_ref().name;

        let name = if name.is_some() {
            name.as_ref().unwrap()
        } else {
            return Err(Status::data_loss("名字必须提供"));
        };

        if !view::can_manage_write(&account_id, &role_group, &DATAS_MANAGE_ID.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let data_manager = majordomo_arc
            .get_manager_by_id(DATAS_MANAGE_ID)
            .await
            .unwrap();
        let associated_manager = majordomo_arc.get_manager_by_id(*manage_id).await.unwrap();

        let local_name = doc! {
            name.language.clone(): name.name.clone()
        };

        if let Some(mut new_entity_doc) = make_new_entity_document(&data_manager).await {
            new_entity_doc.insert(NAME_MAP_FIELD_ID.to_string(), local_name);
            new_entity_doc.insert(DATAS_DATA_TYPE_FIELD_ID.to_string(), data_type);
            new_entity_doc.insert(DATAS_OWNER_MANAGE_ID_FIELD_ID.to_string(), manage_id);
            new_entity_doc.insert(DATAS_OWNER_ENTITY_ID_FIELD_ID.to_string(), entity_id);

            let mut data_id = None;
            let result = data_manager
                .sink_entity(&mut new_entity_doc, &account_id, &role_group)
                .and_then(|id| {
                    data_id = Some(id.clone());
                    let query_doc = doc! {"_id": entity_id.clone()};
                    let modify_doc = doc! {DATAS_FIELD_ID.to_string(): id.clone()};

                    associated_manager.push_entity_array_field(query_doc, modify_doc, &account_id)
                })
                .await;

            match result {
                Ok(_r) => Ok(Response::new(NewDataResponse {
                    result: data_id.unwrap().clone(),
                })),
                Err(e) => Err(Status::aborted(format!(
                    "{} {}",
                    e.operation(),
                    e.details()
                ))),
            }
        } else {
            Err(Status::aborted("新增数据失败。"))
        }
    }
}

