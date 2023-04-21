use async_trait::async_trait;
use bson::doc;
use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use managers::traits::ManagerTrait;
use request_utils::request_account_context;

use tonic::{Request, Response, Status};
use view;

use crate::UnaryResponseResult;

#[async_trait]
pub trait HandleGetManageEntryCount {
    /// 取得管理记录数量
    async fn handle_get_manage_entry_count(
        &self,
        request: Request<GetManageEntryCountRequest>,
    ) -> UnaryResponseResult<GetManageEntryCountResponse> {
        let (account_id, _groups, role_group) =
            request_account_context(&request.metadata());

        let manage_id = &request.get_ref().manage_id;

        if !view::can_collection_read(&account_id, &role_group, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可读权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc.get_manager_by_id(*manage_id).await.unwrap();

        let result = manager.get_entry_counts(doc! {}).await;

        match result {
            Ok(r) => Ok(Response::new(GetManageEntryCountResponse { count: r })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}
