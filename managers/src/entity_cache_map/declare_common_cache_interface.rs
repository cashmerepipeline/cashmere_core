
#[macro_export]
macro_rules! declare_common_cache_interface {
    ($manager_type:ty, $entity_cache_map:ident, $manage_id:ident) => {
        #[async_trait]
        impl EntityCacheInterface for $manager_type {
            fn has_cache(&self) -> bool {
                true
            }

            async fn get_cache(&self) -> Option<MEntityCacheMap> {
                unsafe {
                    if $entity_cache_map.get().is_some() {
                        return Some($entity_cache_map.get().unwrap().clone());
                    }

                    let cache = if let Some(r) = cache_init_cache($manage_id).await {
                        r
                    } else {
                        error!("{} {}", t!("初始化缓存失败"), $manage_id);
                        return None;
                    };

                    Some($entity_cache_map.get_or_init(|| cache).clone())
                }
            }
        }
    };
}
