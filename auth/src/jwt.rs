/*
Author: 闫刚 (yes7rose@sina.com)
auth.rs (c) 2020
Desc: 授权生成
Created:  2020-10-11T02:48:38.636Z
Modified: !date!
*/

use argon2::{self, Config};
use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, crypto::verify, encode};
use serde::{Deserialize, Serialize};

use configs;
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

// 加密密码
pub async fn hash_password(passwd: &String) -> Option<String> {
    let password = passwd.as_bytes();
    let now = Utc::now().to_rfc3339();
    let salt = now.as_bytes();

    match argon2::hash_encoded(password, &salt, &Config::default()) {
        Ok(h) => Some(h),
        Err(_) => None,
    }
}

// 校验密码
pub async fn verify_passwd(passwd: &String, hash: &String) -> Option<bool> {
    let passwd = passwd.as_bytes();
    match argon2::verify_encoded(hash, &passwd) {
        Ok(r) => Some(r),
        Err(_) => None,
    }
}

// 生成jwt token
pub async fn gen_jwt_token(
    phone: &String,
    name: &String,
    orgnizations: &Vec<String>,
    departments: &Vec<String>,
    roles: &Vec<String>,
) -> Option<String> {
    let configs = configs::get_configs();
    let secret_code = configs.server.secret_code.as_bytes();

    let mut header = Header::new(Algorithm::HS512);
    header.kid = Some(phone.clone().to_owned());
    let claims: Claims = Claims {
        aud: phone.clone(),
        name: name.clone(),
        org: orgnizations.clone(),
        dpt: departments.clone(),
        exp: (Utc::now().timestamp() + 60 * 60 * 24 * 7) as usize,
        iat: Utc::now().timestamp() as usize,
        iss: "grpc.cashmere.swb".to_string(),
        sub: "cashmere".to_string(),
        roles: roles.clone(),
    };

    let token = encode(&header, &claims, &EncodingKey::from_secret(secret_code)).unwrap();

    Some(token)
}

pub async fn validate_jwt_token(token: &String) -> bool {
    let configs = configs::get_configs();
    let secret_code = configs.server.secret_code.as_bytes();

    let splits: Vec<&str> = token.split('.').collect();
    let signature = splits[2];
    let message = format!("{}.{}", splits[0], splits[1]);

    let result = verify(
        signature,
        &message,
        &DecodingKey::from_secret(secret_code.as_ref()),
        Algorithm::HS512,
    )
    .unwrap();

    result
}

// 比较token aud 和 phone
pub async fn validate_is_owner(id: &String, token: &String) -> bool {
    let c = match get_claims(token) {
        Some(c) => c.aud,
        None => return false,
    };

    return id.as_bytes() == c.as_bytes();
}

pub async fn validate_is_root(token: &String) -> bool {
    let r = match get_claims(token) {
        Some(c) => c.roles,
        None => return false,
    };

    if r.contains(&"1000000".to_string()) || r.contains(&"1000000".to_string()) {
        return true;
    }

    false
}

// pub async fn validate_is_super_admin(token: &String) -> bool {
//     let (roles, groupid) = match get_claims(token).await {
//         Some(c) => (c.roles, c.groupid),
//         None => return false,
//     };
//
//     let admin_group_id = configs::get_configs().database.admin_group_id.to_string();
//
//     if roles.contains(&"super".to_string())
//         && groupid.as_bytes() == admin_group_id.as_bytes()
//     {
//         return true;
//     }
//
//     false
// }
//
// pub async fn validate_is_company_member(token: &String, company_id: &String) -> bool {
//     let token_comp_id = match get_claims(token).await {
//         Some(c) => c.groupid,
//         None => return false,
//     };
//
//     company_id.as_bytes() == token_comp_id.as_bytes()
// }



// pub async fn get_claims_company_id(token: &String) -> Option<String> {
//     let company_id = match get_claims(token).await {
//         Some(c) => c.groupid,
//         None => return None,
//     };
//
//     Some(company_id)
// }

// claims utils
pub fn get_claims(token: &String) -> Option<Claims> {
    let configs = configs::get_configs();
    let secret_code = configs.server.secret_code.as_bytes();

    match jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret_code),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(c) => Some(c.claims),
        Err(_err) => None,
    }
}
