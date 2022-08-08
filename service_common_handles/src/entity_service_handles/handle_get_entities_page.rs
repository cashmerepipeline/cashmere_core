use async_trait::async_trait;
use bson::{doc, Document};
use tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use managers::traits::ManagerTrait;
use view::add_query_filters;

use crate::UnaryResponseResult;

#[async_trait]
pub trait HandleGetEntitiesPage {
    /// 取得产品分页
    async fn handle_get_entities_page(
        &self,
        request: Request<GetEntitiesPageRequest>,
    ) -> UnaryResponseResult<GetEntitiesPageResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

        let manage_id = &request.get_ref().manage_id;
        let page_index = &request.get_ref().page_index;
        let conditions = &request.get_ref().conditions;

        // 管理可读性检查
        if !view::can_manage_read(&account_id, &groups, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有管理可读权限"));
        }

        // 集合可读性检查
        if !view::can_collection_read(&account_id, &groups, &manage_id.to_string()).await {
            return Err(Status::unauthenticated("用户不具有集合可读权限"));
        }

        let majordomo_arc = get_majordomo().await;
        let manager = majordomo_arc.get_manager_by_id(*manage_id).await.unwrap();

        let mut conditions_doc = bson::to_document(conditions).ok().or(Some(doc! {}));
        // TODO: 条件只支持几种固定格式，需要安全性检查

        // 加入可读过滤项
        if let Some(filter_doc) =
            add_query_filters(&account_id.to_string(), &groups, &manage_id.to_string()).await
        {
            filter_doc
                .iter()
                .for_each(|(k, v)| {conditions_doc.as_mut().unwrap().insert(k, v);});
        };

        // TODO: 字段可见性过滤

        let result = manager
            // .get_entities_by_page(*page_index, &None, &Some(conditions_doc))
            .get_entities_by_page(*page_index, &None, &conditions_doc)
            .await;


        match result {
            Ok(entities) => Ok(Response::new(GetEntitiesPageResponse {
                entities: entities
                    .iter()
                    .map(|x| {
                        let mut bytes: Vec<u8> = Vec::new();
                        x.to_writer(&mut bytes).expect(&*format!("数据损坏:{}", x));
                        bytes
                    })
                    .collect(),
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}
