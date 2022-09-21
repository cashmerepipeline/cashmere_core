/*
    Author: 闫刚 (yes7rose@sina.com)
    account_service_handles.rs (c) 2021
    Desc: 账户管理服务
    Created:  2021-03-29T05:20:05.474Z
    Modified: !date!
*/

use async_trait::async_trait;
use bson::Document;
use chrono::Utc;
use tonic::{Request, Response, Status};

use crate::account_service::*;

use manage_define::manage_ids::ACCOUNTS_MANAGE_ID;
use manage_define::manage_ids::PERSONS_MANAGE_ID;
use managers::traits::ManagerTrait;

use crate::UnaryResponseResult;
use manage_define::field_ids::{PERSONS_DEPARTMENTS_FIELD_ID, PERSONS_ORGANIZATIONS_FIELD_ID};

mod handle_login;
mod handle_new_account;
mod handle_add_account_into_group;
mod handle_remove_account_from_group;

pub use handle_login::HandleLogin;
pub use handle_new_account::HandleNewAccount;
pub use handle_add_account_into_group::HandleAddAccountIntoGroup;
pub use handle_remove_account_from_group::HandleRemoveAccountFromGroup;
