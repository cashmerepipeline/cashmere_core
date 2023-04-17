use std::collections::BTreeMap;
use std::sync::Arc;

use crate::{Conversation, TerminalDelegate};

pub type ConversationsMap = BTreeMap<String, Conversation>;
pub type TerminalDelegatesMap = BTreeMap<String, TerminalDelegate>;
