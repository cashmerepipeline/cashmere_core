/*
Author: 闫刚 (yes7rose@sina.com)
managers.rs (c) 2020
Desc: 管理器包裹
Created:  2020-11-29T04:53:15.516Z
Modified: !date!
*/
use dependencies_sync::rust_i18n::{self};
rust_i18n::i18n!("locales");

pub use majordomo::*;
pub use majordomo_arc::*;
pub use managers_map::*;

mod majordomo;
mod majordomo_arc;
mod managers_map;



