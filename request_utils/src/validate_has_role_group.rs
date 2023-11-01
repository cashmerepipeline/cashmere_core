use auth::{get_auth_token, get_claims_account_and_roles, get_current_role};
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::{Request, Status};

/// 检查是否有角色组
/// 在验证token之后，验证是否有角色组, 不单独使用
pub async fn validate_has_role_group<T>(request: Request<T>) -> Result<Request<T>, Status> {
    let metadata = request.metadata();
    let auth_token = match get_auth_token(metadata) {
        Some(token) => token,
        None => return Err(Status::unauthenticated(t!("请先登录"))),
    };

    let (_account_id, groups) =
        match get_claims_account_and_roles(&auth_token) {
            Some(r) => r,
            None => return Err(Status::unauthenticated(t!("请先登录"))),
        };

    // 检查当前使用角色组是否在组表中存在
    if let Some(ref r) = get_current_role(metadata) {
        if !groups.contains(r) {
            return Err(Status::unauthenticated(t!("没有授权组")));
        }
    } else {
        return Err(Status::data_loss(t!("需要指定所用角色组")));
    };

    Ok(request)
}
