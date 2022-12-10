use parking_lot::RwLock;
use std::sync::Arc;
use manage_define::cashmere::FileInfo;

use crate::DataServer;
use crate::upload_delegators_pool::{get_upload_delegator_pool, UploadDelegatorsPool};
use crate::UploadDelegator;

impl DataServer {
    pub fn get_upload_delegator (&self, file_info: &FileInfo) -> Option<Arc<UploadDelegator>>
    {
        let delegators_pool_arc = get_upload_delegator_pool(Some(self.max_upload_connections_number));
        let delegator_pool = delegators_pool_arc.write();

        delegator_pool.request_delegator(file_info)
    }

    pub fn return_back_delegator (&self, mut delegator: Arc<UploadDelegator>){
        let delegator_pool_arc = get_upload_delegator_pool(Some(self.max_upload_connections_number));
        let delegator_pool = delegator_pool_arc.write();
        
        // TODO: 重置上传代理

        delegator_pool.return_back_delegator(delegator);
    }
}