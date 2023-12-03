
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::{Request, Status};





use request_utils::request_account_context;

/// zh: 验证组有效性
pub async fn validate_is_login<T>(request: Request<T>) -> Result<Request<T>, Status> {
    let (account_id, _groups, role_group) = request_account_context(request.metadata())?;
    
    if account_id == "anonymous" {
        return Err(Status::unauthenticated(t!("需要登录")));
    }

    if role_group == "anonymous" {
        return Err(Status::unauthenticated(t!("需要登录后指定当前操作组")));
    }

    Ok(request)
}

