// 异步线程限制
#![recursion_limit = "256"]

use dependencies_sync::rust_i18n::{self, i18n};
i18n!("locales");

pub use manager_trait::ManagerTrait;
pub use manager::*;
pub use manager_inner::*;
pub use get_tokenizers::*;

pub mod manages_manager;

pub mod areas_manager;
pub mod categaries_manager;
pub mod tags_manager;
pub mod comments_manager;
pub mod groups_manager;
pub mod persons_manager;
pub mod templates_manager;
pub mod view_rules_manager;

pub mod country_codes_manager;
pub mod language_codes_manager;
pub mod phone_area_codes_manager;

pub mod utils;

mod macros;

pub(crate) mod schema;
pub mod manager_trait;
mod manager_inner;
mod manager;
mod entity_cache_map;
// pub mod tag_types_manager;
// pub mod show_settings_manager;

mod get_text_options;
mod get_tokenizers;
