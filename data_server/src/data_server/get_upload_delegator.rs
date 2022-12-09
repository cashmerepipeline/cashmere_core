use parking_lot::RwLock;
use std::sync::Arc;

use crate::DataServer;
use crate::upload_delegators_pool::get_upload_delegator_pool;
use crate::UploadDelegator;

impl DataServer {
    pub fn get_file_upload_delegator(&self) -> Option<Arc<RwLock<UploadDelegator>>>{
        let receiver_pool = get_upload_delegator_pool(self.max_upload_connections_number);
        let receiver_pool_arc = receiver_pool.write();

        receiver_pool_arc.request_delegator()
    }
}