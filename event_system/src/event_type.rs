use std::collections::BTreeMap;
use tokio::sync::broadcast::{Receiver, Sender};

use uuid::Uuid;

use crate::event::Event;
use crate::event_message::EventMessageFieldDataType;
use crate::event_message::EventMessageFieldInfo;

#[derive(Debug, Clone)]
pub struct EventType {
    pub uid: String,
    pub name: String,
    pub schema: BTreeMap<String, EventMessageFieldInfo>,
    pub description: String,
    pub sender: Sender<Event>
}

impl EventType {
    pub fn new(new_name: String, sender: Sender<Event>, uid: Option<String>, description: Option<String>, schema: Option<BTreeMap<String, EventMessageFieldInfo>>) -> EventType {
        EventType {
            uid: uid.or(Some(Uuid::new_v4().urn().to_string())).unwrap(),
            name: new_name,
            description: description.or(Some("".to_string())).unwrap(),
            schema: schema.or(Some(BTreeMap::new())).unwrap(),
            sender: sender
        }
    }

    pub fn subscribe(&self) -> Receiver<Event>{
        self.sender.subscribe()
    }

    pub fn get_sender(&self) -> Sender<Event> {
        return self.sender.clone()
    }
}
