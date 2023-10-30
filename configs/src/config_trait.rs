use dependencies_sync::log;
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::serde::Serialize;

use crate::configs_map::get_configs_map;

pub trait ConfigTrait {
    fn name() -> &'static str;
}
