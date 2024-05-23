use dependencies_sync::{
    log,
    rust_i18n::{self, t},
};

use cash_result::{operation_failed, OperationResult};

use crate::get_entity_by_id;

use super::get_hard_coded_cache_map;

pub async fn refresh_entity_hard_coded_cache(
    manage_id: &str,
    entity_id: &str,
) -> Result<(), OperationResult> {
    // zh: 冲数据库获取最新
    let entity = if let Ok(r) = get_entity_by_id(manage_id, entity_id, &[], &[]).await {
        r
    } else {
        log::error!("{}: {}", t!("取得硬编码实体失败"), manage_id);
        return Err(operation_failed(
            "refresh_hard_coded_cache",
            format!("{}: {}", t!("取得硬编码实体失败"), manage_id),
        ));
    };

    // zh: 更新缓存
    let map_arc = get_hard_coded_cache_map(manage_id).await.unwrap();
    let mut map = map_arc.write();
    map.insert(entity_id.to_string(), entity);

    Ok(())
}
