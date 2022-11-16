use std::sync::Arc;

use parking_lot::RwLock;

use super::{event::Event, event_queue_map::get_events_queue_map, types::EventsQueueMap};

#[derive(Default)]
pub struct EventDispatcher;

/// 事件分发器
impl EventDispatcher {
    pub fn dispatch(&self, event: Event) {
        // 1. get event type listeners

        // 2. send event
        println!("{}", "sending event");
    }
}
