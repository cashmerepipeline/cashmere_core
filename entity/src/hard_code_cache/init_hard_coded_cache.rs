use std::sync::Arc;

use cash_result::{operation_failed, operation_succeed, OperationResult};
use dependencies_sync::bson::doc;
use dependencies_sync::indexmap::IndexMap;
use dependencies_sync::log::{self, debug, error, info};
use dependencies_sync::parking_lot::RwLock;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tokio_stream::StreamExt;
use manage_define::general_field_ids::ID_FIELD_ID;

use super::HardCodedCacheMap;

pub async fn init_hard_coded_cache(manage_id: &str) -> Result<HardCodedCacheMap, OperationResult> {
    log::debug!("{}: {}", t!("开始初始化硬编码缓存"), manage_id,);

    let new_map = Arc::new(RwLock::new(IndexMap::new()));
    {
        let cursor = crate::get_query_cursor(manage_id, doc! {}, &[], None, None, 0).await;

        if let Ok(mut r) = cursor {
            while let Some(d) = r.next().await {
                if let Ok(d) = d {
                    let id = d.get_str(ID_FIELD_ID.to_string()).unwrap().to_string();
                    let mut m = new_map.write();
                    m.insert(id, d);
                }
            }
        } else {
            error!("{}: {}", t!("初始化硬编码缓存失败"), manage_id,);

            return Err(operation_failed(
                "init_hard_coded_cache",
                t!("初始化硬编码缓存失败"),
            ));
        }
    }

    info!("{}: {}", t!("初始化硬编码缓存完成"), manage_id,);

    Ok(new_map.clone())
}
