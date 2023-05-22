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

use crate::UnaryResponseResult;

#[async_trait]
pub trait HandleEditEntityArrayFieldRemoveItems {
    /// 编辑区域
    async fn handle_edit_entity_array_field_remove_items(
        &self,
        request: Request<EditEntityArrayFieldRemoveItemsRequest>,
    ) -> UnaryResponseResult<EditEntityArrayFieldRemoveItemsResponse> {
        let (account_id, _groups, role_group) =
            request_account_context(request.metadata());

        let manage_id = &request.get_ref().manage_id;
        let entity_id = &request.get_ref().entity_id;
        let field_id = &request.get_ref().field_id;
        // bson bytes {field_id:[items]}
        let items = &request.get_ref().items;

        if !view::can_collection_write(&account_id, &role_group, &manage_id.to_string()).await {
            return Err(Status::permission_denied("用户不具有集合可写权限"));
        }

        if !view::can_entity_write(&account_id, &role_group, &manage_id.to_string()).await {
            return Err(Status::permission_denied("用户不具有实体可写权限"));
        }
        if !view::can_field_write(&account_id, &role_group, &manage_id.to_string(), field_id).await
        {
            return Err(Status::permission_denied("用户不具有属性可写权限"));
        }

        let b_items = match Document::from_reader(items.reader()) {
            Ok(v) => {
                let t_v = v.get_array(field_id);
                if t_v.is_ok() {
                    t_v.unwrap().clone()
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

        let mut modify_doc = Document::new();
        modify_doc.insert(field_id, doc! {"$in":b_items.clone()});

        let result = manager
            .remove_from_array_field(query_doc, modify_doc, &account_id)
            .await;

        match result {
            Ok(_r) => Ok(Response::new(EditEntityArrayFieldRemoveItemsResponse {
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
