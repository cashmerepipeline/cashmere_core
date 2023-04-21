use tonic::metadata::MetadataMap;

/// 获取请求上下文，包括：account_id, groups, role_group
pub fn request_account_context(metadata: &MetadataMap) -> (String, Vec<String>, String) {

    let token = auth::get_auth_token(metadata).unwrap();
    let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
    let role_group = auth::get_current_role(metadata).unwrap();

    (account_id, groups, role_group)
}

