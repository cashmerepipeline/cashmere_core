use std::fmt::format;
use async_trait::async_trait;
use bson::{doc, Document};
use prost::bytes::Buf;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use view;

use crate::UnaryResponseResult;

#[async_trait]
pub trait HandleEditEntityMapField {
    /// 编辑区域
    async fn handle_edit_entity_map_field(
        &self,
        request: Request<EditEntityMapFieldRequest>,
    ) -> UnaryResponseResult<EditEntityMapFieldResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let manage_id = &request.get_ref().manage_id;
        let entity_id = &request.get_ref().entity_id;
        let field_id = &request.get_ref().field_id;
        let key = &request.get_ref().key;
        // bson bytes {key:new_value}
        let new_value = &request.get_ref().new_value;

        if !view::can_collection_write(&account_id, &role_group, &manage_id.to_string()).await {
            return Err(Status::permission_denied("用户不具有集合可写权限"));
        }

        if !view::can_entity_write(&account_id, &role_group, &manage_id.to_string()).await {
            return Err(Status::permission_denied("用户不具有实体可写权限"));
        }

        if !view::can_field_write(&account_id, &role_group, &manage_id.to_string(), field_id).await {
            return Err(Status::permission_denied(format!("用户不具有属性可写权限{}-{}-{}", manage_id, entity_id, field_id)));
        }

        let value =
            match Document::from_reader(new_value.reader()) {
                Ok(ref v) => {
                    // 属性key一致
                    let t_v = v.get(key);
                    if t_v.is_some() {
                        t_v.unwrap().clone()
                    } else {
                        return Err(Status::data_loss("新值不能为空"));
                    }
                },
                Err(_) => return Err(Status::data_loss("新值不能为空"))
            };

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(*manage_id)
            .await
            .unwrap();

        let query_doc = doc! {
            ID_FIELD_ID.to_string():entity_id,
        };
        let mut modify_doc = Document::new();
        modify_doc.insert(format!("{}.{}", field_id, key), value.clone());

        let result = manager
            .update_entity_field(query_doc, modify_doc, &account_id)
            .await;

        match result {
            Ok(_r) => Ok(Response::new(EditEntityMapFieldResponse {
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


