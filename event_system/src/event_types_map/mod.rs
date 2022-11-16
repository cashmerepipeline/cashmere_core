/*
    可监听事件表
 */

use std::collections::BTreeMap;
use std::sync::Arc;

use parking_lot::RwLock;

pub use deregister_event_type::*;
pub use event_types_map::get_event_types_map;
pub use get_event_type::*;
pub use get_event_types::*;
pub use register_event_type::*;

use crate::types::{EventTypeEmittersMap, EventsQueueMap, EventTypesMap};

mod register_event_type;
mod deregister_event_type;
pub(crate) mod get_event_types;
mod event_types_map;
mod get_event_type;

