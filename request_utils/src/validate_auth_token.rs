use std::any::{Any, TypeId};

use auth::{get_auth_token, get_claims_account_and_roles, get_current_role};
use auth::jwt::{get_claims, validate_jwt_token};

use dependencies_sync::chrono::Utc;
use dependencies_sync::tonic::{Request, Status};
use dependencies_sync::log::debug;
use dependencies_sync::rust_i18n::{self, t};

use manage_define::cashmere::GetPhoneAreaCodesRequest;

///检查授权token
pub async fn validate_auth_token<T>(request: Request<T>) -> Result<Request<T>, Status> {
    // token检查
    let auth_meta = request.metadata();
    let auth_token = match get_auth_token(auth_meta) {
        Some(token) => token,
        None => return Err(Status::unauthenticated(t!("请先登录"))),
    };

    // jwt校验
    if !validate_jwt_token(&auth_token) {
        return Err(Status::unauthenticated(format!(
            "{}, {}",
            t!("权限验证错误"),
            t!("请重新登录")
        )));
    }

    // 用户校验
    let (_account_id, _groups) =
        match get_claims_account_and_roles(&auth_token) {
            Some(r) => r,
            None => return Err(Status::unauthenticated(t!("请先登录"))),
        };
    
    // jwt声明校验
    let claims = match get_claims(&auth_token) {
        Some(r) => r,
        None => return Err(Status::unauthenticated(t!("请先登录"))),
    };

    // 过期校验
    let now = Utc::now().timestamp();
    if now > claims.exp as i64 {
        return Err(Status::cancelled(format!(
            "{}, {}",
            t!("登录已过期"),
            t!("请重新登录")
        )));
    }

    Ok(request)
}
