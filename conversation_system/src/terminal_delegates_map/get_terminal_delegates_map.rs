use std::collections::BTreeMap;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::types::TerminalDelegatesMap;

///  会话表
static mut TERMINAL_PROXIES_MAP: Option<Arc<RwLock<TerminalDelegatesMap>>> = None;

/// 取得发送者映射
pub fn get_terminal_deleagates_map() -> Arc<RwLock<TerminalDelegatesMap>> {
    if let Some(_map) = unsafe { TERMINAL_PROXIES_MAP.as_ref() } {
        _map.clone()
    } else {
        let _map = Arc::new(RwLock::new(BTreeMap::new()));
        unsafe {
            TERMINAL_PROXIES_MAP = Some(_map.clone());
        }
        _map
    }
}



