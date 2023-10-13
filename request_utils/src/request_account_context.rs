use dependencies_sync::tonic::metadata::MetadataMap;

/// 获取请求上下文，包括：account_id, groups, role_group
pub fn request_account_context(metadata: &MetadataMap) -> (String, Vec<String>, String) {
    let token = auth::get_auth_token(metadata).unwrap();
    println!("token: {:?}", token);
    let (account_id, groups) = auth::get_claims_account_and_roles(&token)
        .unwrap_or(("anonymous".to_string(), vec!["anonymous".to_string()]));
    println!("account_id: {:?}, groups: {:?}", account_id, groups);

    let role_group = auth::get_current_role(metadata).unwrap();
    println!("role_group: {:?}", role_group);

    (account_id, groups, role_group)
}
