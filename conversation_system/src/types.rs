use std::collections::{BTreeMap, VecDeque};
use std::sync::Arc;

use parking_lot::RwLock;

use crate::{Conversation, TerminalDelegate};

pub type ConversationsMap = BTreeMap<String, Conversation>;
pub type TerminalDelegatesMap = BTreeMap<String, TerminalDelegate>;
