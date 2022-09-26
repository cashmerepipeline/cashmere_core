/*
Project: grpc_server
Creator: 闫刚
Create time: 2020-09-10 08:11
Introduction:
*/

pub const ROLE_GROUP_NAME: &str = "role_group";

pub mod check;
pub mod jwt;
mod get_auth_token;
mod get_claims_account_and_roles;
mod get_current_role_group;
mod test;

use tonic::metadata::MetadataMap;

pub use get_auth_token::get_auth_token;
pub use get_claims_account_and_roles::get_claims_account_and_roles;
pub use get_current_role_group::get_current_role;
