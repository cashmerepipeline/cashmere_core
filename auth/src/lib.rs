/*
Project: grpc_server
Creator: 闫刚
Create time: 2020-09-10 08:11
Introduction:
*/
use dependencies_sync::rust_i18n::{self};

rust_i18n::i18n!("locales");

pub mod jwt;
mod get_auth_token;
mod get_claims_account_and_roles;
mod get_current_role_group;
mod test;
mod hash_password;
mod constant_names;

pub use get_auth_token::get_auth_token;
pub use get_claims_account_and_roles::get_claims_account_and_roles;
pub use get_current_role_group::get_current_role;
pub use hash_password::hash_password;
pub use constant_names::*;