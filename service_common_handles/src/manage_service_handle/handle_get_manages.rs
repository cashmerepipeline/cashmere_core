use async_trait::async_trait;
use bson::doc;
use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::general_field_ids::{ID_FIELD_ID, NAME_MAP_FIELD_ID};
use managers::traits::ManagerTrait;
use request_utils::request_account_context;

use dependencies_sync::tonic::{Request, Response, Status};
use view::can_manage_read;

#[async_trait]
pub trait HandleGetManages {
    /// 取得管理
    async fn handle_get_manages(
        &self,
        request: Request<GetManagesRequest>,
    ) -> Result<Response<GetManagesResponse>, Status> {
        let (account_id, _groups, role_group) =
            request_account_context(request.metadata());

        let managers_ids: Vec<i32> = get_majordomo().await.get_manager_ids().await;

        let mut result: Vec<Manage> = Vec::new();
        for id in managers_ids {
            let manager = get_majordomo().await.get_manager_by_id(id).await.unwrap();
            let doc = manager.get_manage_document().await.read().clone();

            let mut name_map: Vec<u8> = Vec::new();
            doc.get_document(NAME_MAP_FIELD_ID.to_string())
                .unwrap()
                .to_writer(&mut name_map)
                .unwrap();

            let m = Manage {
                manage_id: doc
                    .get_str(ID_FIELD_ID.to_string())
                    .unwrap()
                    .parse::<i32>()
                    .unwrap(),
                name_map,
            };

            // 可见性过滤，返回可读管理
            if can_manage_read(&account_id, &role_group, &id.to_string()).await {
                result.push(m);
            }
        }

        Ok(Response::new(GetManagesResponse { manages: result }))
    }
}
