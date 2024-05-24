use cash_result::OperationResult;

use super::{get_manage_hard_code_cache_map, init_hard_coded_cache, HardCodedCacheMap};

/// 取得硬编码缓存
pub async fn get_hard_coded_cache_map(
    manage_id: &str,
) -> Result<HardCodedCacheMap, OperationResult> {
    let cache_map = {
        let m_cache_map_arc = get_manage_hard_code_cache_map();
        let m_cache_map = m_cache_map_arc.read();
        m_cache_map.get(manage_id).cloned()
    };

    let result = if let Some(m) = cache_map {
        m
    } else {
        let m = init_hard_coded_cache(manage_id).await?;
        let m_cache_map_arc = get_manage_hard_code_cache_map();
        let mut m_cache_map = m_cache_map_arc.write();
        m_cache_map.insert(manage_id.to_string(), m.clone());
        m
    };

    Ok(result)
}
