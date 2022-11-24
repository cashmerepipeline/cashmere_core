use std::sync::Arc;

use crate::TerminalDelegate;

pub fn get_terminal_delegate(terminal_id: &String) -> Arc<TerminalDelegate> {

    Arc::new(TerminalDelegate{})
}
