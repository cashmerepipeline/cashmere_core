use dependencies_sync::tonic::async_trait;
use dependencies_sync::bson::{self, doc};

use dependencies_sync::tonic::{Request, Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;

use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use view::{add_query_filters};
use service_utils::types::UnaryResponseResult;

#[async_trait]
pub trait HandleListSpecsPrefabs {
    /// 取得产品分页
    async fn handle_list_specs_prefabs(
        &self,
        request: Request<ListSpecsPrefabsRequest>,
    ) -> UnaryResponseResult<ListSpecsPrefabsResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, _groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let specs_id = &request.get_ref().specs_id;

        let manage_id = PREFABS_MANAGE_ID;

        let majordomo_arc = get_majordomo();
        let manager = majordomo_arc.get_manager_by_id(manage_id).unwrap();

        // 可读性过滤, 没有设置过滤即不可读
        // TODO: 根据组改写，加入可读过滤项
        let mut matches = doc! {};
        if let Some(filter_doc) =
            add_query_filters(&account_id.to_string(), &role_group, &manage_id.to_string()).await
        {
            filter_doc.iter().for_each(|(k, v)| {
                matches.insert(k, v);
            });
        } else {
            return Err(Status::unauthenticated(
                "没有可读描写字段，用户不具有集合可读权限",
            ));
        };

        let filter_doc = doc! {
          PREFABS_SPECS_ID_FIELD_ID.to_string(): specs_id.clone(),
        };

        let result = manager.get_entities_by_filter(&Some(filter_doc)).await;

        match result {
            Ok(entities) => Ok(Response::new(ListSpecsPrefabsResponse {
                prefabs: entities.iter().map(|x| bson::to_vec(x).unwrap()).collect(),
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}
