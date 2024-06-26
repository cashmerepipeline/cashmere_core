// 异步线程限制
#![recursion_limit = "256"]

use dependencies_sync::rust_i18n::{self, i18n};
i18n!("locales");

pub use manager_trait::ManagerInterface;
pub use manager::*;

pub mod manages_manager;

pub mod areas_manager;

pub mod categaries_manager;
pub mod tags_manager;
pub mod comments_manager;
pub mod recommends_manager;

pub mod groups_manager;
pub mod persons_manager;
pub mod templates_manager;
pub mod view_rules_manager;

pub mod country_codes_manager;
pub mod language_codes_manager;
pub mod phone_area_codes_manager;
pub mod calendars_manager;
pub mod calendar_books_manager;

pub mod utils;

mod macros;

pub mod manager_trait;
// mod manager_inner;
mod manager;

pub mod hard_coded_cache_interface;
pub mod manage_interface;
pub mod entity_interface;

// pub mod show_settings_manager;

