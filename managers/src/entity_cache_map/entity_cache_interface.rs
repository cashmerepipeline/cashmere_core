
use dependencies_sync::tonic::async_trait;

use super::{MEntityCacheMap};

#[async_trait]
pub trait EntityCacheInterface{
  /// zh: 默认没有缓存
  fn has_cache(&self) -> bool{
    false
  }

  /// zh: 默认没有缓存
  async fn get_cache(&self) -> Option<MEntityCacheMap>{
    None
  }
}