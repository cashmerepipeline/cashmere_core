/*
Author: 闫刚 (yes7rose@sina.com)
lib.rs (c) 2021
Desc: 服务处理定义
Created:  2021-02-14T09:31:29.747Z
Modified: !date!
*/

#[macro_use]
extern crate rust_i18n;
i18n!("locales");

pub use types::*;


pub mod area_service_handles;

pub mod data_service_handles;
pub mod stage_service_handles;
pub mod specses_service_handles;
pub mod prefab_service_handles;

pub mod entity_service_handles;
pub mod group_service_handles;
pub mod manage_service_handle;
pub mod name_service_handles;
pub mod name_utils;
pub mod language_code_handles;
pub mod view_rules_service_handles;
pub mod country_service_handles;

mod types;
