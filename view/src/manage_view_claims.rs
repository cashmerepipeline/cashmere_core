use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ManageViewClaims {
    // 管理
    manage: String,
    // 用户名
    account_id: String,
    // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    exp: usize,
    // Optional. Issued at (as UTC timestamp)
    iat: usize,
    // Optional. Issuer
    iss: String,
    // Optional. Subject (whom token refers to)
    sub: String,
    // 权限
    // view_map: LinkedHashMap<String, String>,
}
