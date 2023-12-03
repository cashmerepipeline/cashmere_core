use dependencies_sync::{tonic::{metadata::MetadataMap, Status}, rust_i18n::{self, t}};

/// 获取请求上下文，包括：account_id, groups, role_group
pub fn request_account_context(
    metadata: &MetadataMap,
) -> Result<(String, Vec<String>, String), Status> {
    let token = if let Some(r) = auth::get_auth_token(metadata) {
        r
    } else {
        return Err(Status::unauthenticated(format!(
            "{}",
            t!("读取metadata auth token失败")
        )));
    };

    let (account_id, groups) = auth::get_claims_account_and_roles(&token)
        .unwrap_or(("anonymous".to_string(), vec!["anonymous".to_string()]));

    let role_group = if let Some(r) = auth::get_current_role(metadata) {
        r
    } else {
        return Err(Status::unauthenticated(format!(
            "{}",
            t!("没有指定角色组")
        )));
    };

    Ok((account_id, groups, role_group))
}
