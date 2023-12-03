
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::{Request, Status};





use request_utils::request_account_context;

/// zh: 验证组有效性
pub async fn validate_role_group<T>(request: Request<T>) -> Result<Request<T>, Status> {
    let (_account_id, _groups, role_group) = request_account_context(request.metadata())?;
    if _groups.contains(&role_group) {
        Ok(request)
    } else {
        Err(Status::invalid_argument(format!(
            "{}: {}",
            t!("不是有校组"),
            role_group
        )))
    }
}
