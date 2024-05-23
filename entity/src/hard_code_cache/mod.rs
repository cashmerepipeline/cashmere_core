/*
  zh: 针对实体都是硬编码数据设立的缓存。硬编码数据特点： 1. 访问频繁，2. 数据量小，3. 数据变化不频繁。
*/

pub use types::*;
pub use manage_hard_code_cache_map::*;
pub use init_hard_coded_cache::*;
pub use get_cache_map::*;
pub use refresh_hard_coded_cache_map::*;
pub use hard_coded_cache_get_entity::*;
pub use hard_coded_cache_get_entities::*;
pub use hard_coded_cache_get_entity_stream::*;

mod types;
mod manage_hard_code_cache_map;
mod init_hard_coded_cache;
mod get_cache_map;
mod refresh_hard_coded_cache_map;
mod hard_coded_cache_get_entity;
mod hard_coded_cache_get_entities;
mod hard_coded_cache_get_entity_stream;