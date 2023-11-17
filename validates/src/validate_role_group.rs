use dependencies_sync::bson::doc;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::{self, Request, Status};

use majordomo::get_majordomo;
use manage_define::general_field_ids::{ID_FIELD_ID, REMOVED_FIELD_ID};
use manage_define::manage_ids::GROUPS_MANAGE_ID;
use managers::ManagerTrait;
use request_utils::request_account_context;

/// zh: 验证组有效性
pub async fn validate_role_group<T>(request: Request<T>) -> Result<Request<T>, Status> {
    let (_account_id, _groups, role_group) = request_account_context(request.metadata())?;
    if _groups.contains(&role_group) {
        return Ok(request);
    } else {
        return Err(Status::invalid_argument(format!(
            "{}: {}",
            t!("不是有校组"),
            role_group
        )));
    }
}
