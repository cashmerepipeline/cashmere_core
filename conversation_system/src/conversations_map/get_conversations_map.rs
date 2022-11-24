use std::collections::BTreeMap;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::types::ConversationsMap;

///  会话表
static mut CONVERSATIONS_MAP: Option<Arc<RwLock<ConversationsMap>>> = None;

/// 取得发送者映射
pub fn get_conversations_map() -> Arc<RwLock<ConversationsMap>> {
    if let Some(_map) = unsafe { CONVERSATIONS_MAP.as_ref() } {
        _map.clone()
    } else {
        let _map = Arc::new(RwLock::new(BTreeMap::new()));
        unsafe {
            CONVERSATIONS_MAP = Some(_map.clone());
        }
        _map
    }
}


