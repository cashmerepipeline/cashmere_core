/*
Author: 闫刚 (yes7rose@sina.com)
lib.rs (c) 2021
Desc: 服务处理定义
Created:  2021-02-14T09:31:29.747Z
Modified: !date!
*/


use dependencies_sync::rust_i18n::{self, i18n};
i18n!("locales");

pub mod server_status_handles;

pub mod constant_service_handles;
pub mod area_service_handles;
pub mod country_code_service_handles;
pub mod language_code_handles;
pub mod phone_area_code_handles;
pub mod color_service_handles;

pub mod category_service_handle;
pub mod tag_service_handles;

pub mod group_service_handles;

pub mod entity_service_handles;
pub mod template_service_handles;
pub mod manage_service_handle;

pub mod name_service_handles;
pub mod view_rules_service_handles;

pub mod comment_service_handles;

pub mod member_service_handles;

pub mod calendars_service_handles;
pub mod calendar_book_service_handles;