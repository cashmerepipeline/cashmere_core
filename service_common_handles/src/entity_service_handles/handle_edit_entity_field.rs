use async_trait::async_trait;
use bson::{doc, Document};
use prost::bytes::Buf;
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;

use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use request_utils::request_account_context;

use view;

use crate::UnaryResponseResult;

#[async_trait]
pub trait HandleEditEntityField {
    /// 编辑修改实体属性
    async fn handle_edit_entity_field(
        &self,
        request: Request<EditEntityFieldRequest>,
    ) -> UnaryResponseResult<EditEntityFieldResponse> {
        let (account_id, _groups, role_group) =
            request_account_context(request.metadata());

        let manage_id = &request.get_ref().manage_id;
        let entity_id = &request.get_ref().entity_id;
        let field_id = &request.get_ref().field_id;
        // bson bytes {field_id:new_value}
        let new_value = &request.get_ref().new_value;

        if !view::can_collection_write(&account_id, &role_group, &AREAS_MANAGE_ID.to_string()).await
        {
            return Err(Status::permission_denied("用户不具有集合可写权限"));
        }

        // TODO: 描写属性是否存在

        if !view::can_field_write(&account_id, &role_group, &manage_id.to_string(), field_id).await
        {
            return Err(Status::permission_denied("用户不具有属性可写权限"));
        }

        let mut modify_doc = match Document::from_reader(new_value.reader()) {
            Ok(v) => {
                let t_v = v.get(field_id);
                if t_v.is_some() {
                    v
                } else {
                    return Err(Status::data_loss("新值不能为空"));
                }
            }
            Err(_) => return Err(Status::data_loss("新值不能为空")),
        };

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc.get_manager_by_id(*manage_id).await.unwrap();

        let query_doc = doc! {
            ID_FIELD_ID.to_string():entity_id,
        };

        let result = manager
            .update_entity_field(query_doc, &mut modify_doc, &account_id)
            .await;

        match result {
            Ok(_r) => Ok(Response::new(EditEntityFieldResponse {
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
