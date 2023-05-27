use dependencies_sync::tonic::async_trait;
use dependencies_sync::bson::{doc, Document};

use dependencies_sync::prost::bytes::Buf;

use dependencies_sync::tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;

use manage_define::general_field_ids::*;

use managers::traits::ManagerTrait;
use request_utils::request_account_context;

use view;

use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleEditEntityMapField {
    /// 编辑区域
    async fn handle_edit_entity_map_field(
        &self,
        request: Request<EditEntityMapFieldRequest>,
    ) -> UnaryResponseResult<EditEntityMapFieldResponse> {
        let (account_id, _groups, role_group) =
            request_account_context(request.metadata());

        let manage_id = &request.get_ref().manage_id;
        let entity_id = &request.get_ref().entity_id;
        let field_id = &request.get_ref().field_id;
        let key = &request.get_ref().key;
        // bson bytes {key:new_value}
        let new_value = &request.get_ref().new_value;

        

        

        if !view::can_field_write(&account_id, &role_group, &manage_id.to_string(), field_id).await
        {
            return Err(Status::permission_denied(format!(
                "用户不具有属性可写权限{}-{}-{}",
                manage_id, entity_id, field_id
            )));
        }

        let value = match Document::from_reader(new_value.reader()) {
            Ok(ref v) => {
                // 属性key一致
                let t_v = v.get(key);
                if let Some(r) = t_v {
                    r.clone()
                } else {
                    return Err(Status::data_loss("新值不能为空"));
                }
            }
            Err(_) => return Err(Status::data_loss("新值不能为空")),
        };

        let majordomo_arc = get_majordomo();
        let manager = majordomo_arc.get_manager_by_id(*manage_id).unwrap();

        let query_doc = doc! {
            ID_FIELD_ID.to_string():entity_id,
        };
        let mut modify_doc = Document::new();
        modify_doc.insert(format!("{}.{}", field_id, key), value.clone());

        let result = manager
            .update_entity_field(query_doc, &mut modify_doc, &account_id)
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
