use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    // Optional. Audience, 电话好梦
    pub aud: String,
    // 用户名
    pub name: String,
    // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub exp: usize,
    // Optional. Issued at (as UTC timestamp)
    pub iat: usize,
    // Optional. Issuer
    pub iss: String,
    // Optional. Subject (whom token refers to)
    pub sub: String,
    // 组织或者单位
    pub org: Vec<String>,
    // 部门
    pub dpt: Vec<String>,
    // 角色或者身份
    pub roles: Vec<String>,
}
