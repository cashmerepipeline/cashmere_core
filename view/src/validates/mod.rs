/// 各层级权限验证
/// 失败返回grpc未授权状态

pub use validate_collection_can_read::*;
pub use validate_collection_can_write::*;
pub use validate_entity_can_read::*;
pub use validate_entity_can_write::*;
pub use validate_field_can_read::*;
pub use validate_field_can_write::*;
pub use validate_manage_can_read::*;
pub use validate_manage_can_write::*;

mod validate_collection_can_read;
mod validate_collection_can_write;
mod validate_entity_can_read;
mod validate_entity_can_write;
mod validate_field_can_read;
mod validate_field_can_write;
mod validate_manage_can_read;
mod validate_manage_can_write;
