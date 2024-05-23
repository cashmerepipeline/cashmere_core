use cash_result::{add_call_name_to_chain, operation_failed, OperationResult};
use dependencies_sync::{
    log,
    rust_i18n::{self, t},
    tonic::async_trait,
};
use entity::hard_code_cache::HardCodedCacheMap;

use crate::ManagerInterface;

#[async_trait]
pub trait HardCodedInterface
where
    Self: ManagerInterface,
{
    /// zh: 默认没有缓存
    async fn is_hard_coded(&self) -> bool {
        let m = &self.get_manage().await;
        let r = m.read().hard_coded;
        r
    }

    /// zh: 默认没有缓存
    async fn get_hard_coded_cache(&self, manage_id: &str) -> Option<HardCodedCacheMap> {
        match self.is_hard_coded().await {
            true => {
                if let Ok(r) = entity::hard_code_cache::get_hard_coded_cache_map(manage_id).await {
                    Some(r)
                } else {
                    None
                }
            }
            false => None,
        }
    }

    /// zh: 刷新缓存
    async fn refresh_hard_coded_cache(
        &self,
        manage_id: &str,
        entity_id: &str,
    ) -> Result<(), OperationResult> {
        match self.is_hard_coded().await {
            true => {
                if let Err(err) =
                    entity::hard_code_cache::refresh_entity_hard_coded_cache(manage_id, entity_id)
                        .await
                {
                    Err(add_call_name_to_chain(
                        err,
                        "refresh_hard_coded_cache".to_string(),
                    ))
                } else {
                    Ok(())
                }
            }
            false => {
                log::error!("{}: {}", t!("没有缓存"), manage_id);
                return Err(operation_failed("refresh_hard_coded_cache", t!("没有缓存")));
            }
        }
    }
}
