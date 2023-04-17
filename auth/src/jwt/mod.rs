/*
Author: 闫刚 (yes7rose@sina.com)
auth.rs (c) 2020
Desc: 授权生成
Created:  2020-10-11T02:48:38.636Z
Modified: !date!
*/


use configs;

mod claims;
mod verify_passwd;
mod gen_jwt_token;
mod validate_jwt_token;
mod validate_is_owner;
mod validate_is_root;
mod get_claims;

pub use claims::Claims;
pub use crate::hash_password::hash_password;
pub use verify_passwd::verify_passwd;
pub use gen_jwt_token::gen_jwt_token;
pub use validate_jwt_token::validate_jwt_token;
pub use validate_is_owner::validate_is_owner;
pub use validate_is_root::validate_is_root;
pub use get_claims::get_claims;

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
