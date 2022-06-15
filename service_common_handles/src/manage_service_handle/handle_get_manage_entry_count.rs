use async_trait::async_trait;
use bson::{doc, Document};
use chrono::format::parse;
use tonic::{Request, Response, Status};

use crate::cashmere::*;
use crate::UnaryResponseResult;

use majordomo::{self, get_majordomo};
use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use managers::utils::make_new_entity_document;
use view;

#[async_trait]
pub trait HandleGetManageEntryCount {
    /// 取得管理记录数量
    async fn handle_get_manage_entry_count(
        &self,
        request: Request<GetManageEntryCountRequest>,
    ) -> UnaryResponseResult<GetManageEntryCountResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

        let manage_id_str = &request.get_ref().manage_id;

        let mut manage_id:i32 = 0;
        if let id = manage_id_str.parse(){
            manage_id = id.unwrap()
        } else {
            return Err(Status::aborted("请求管理编号不正确。"));
        }

        if !view::can_manage_write(&account_id, &groups, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc
            .get_manager_by_id(manage_id)
            .await
            .unwrap();

        let result = manager
            .get_entry_counts()
            .await;

        match result {
            Ok(r) => Ok(Response::new(GetManageEntryCountResponse {
                count: r
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}