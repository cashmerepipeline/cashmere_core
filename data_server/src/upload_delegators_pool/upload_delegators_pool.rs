use configs::DataServerConfigs;
use parking_lot::RwLock;
use std::sync::Arc;

use crate::UploadDelegator;

/// 接收器池表
static mut UPLOAD_DELEGATORS_POOL: Option<Arc<RwLock<UploadDelegatorsPool>>> = None;

#[derive(Debug)]
pub struct UploadDelegatorsPool {
    delegators: Arc<RwLock<Vec<Arc<RwLock<UploadDelegator>>>>>,
}

pub fn init_upload_delegators_pool(max_upload_number: u16) -> Arc<RwLock<UploadDelegatorsPool>> {
    let mut delegators: Vec<Arc<RwLock<UploadDelegator>>> = vec![];
    for _i in 0..max_upload_number {
        let new_receiver = Arc::new(RwLock::new(UploadDelegator {}));
        delegators.push(new_receiver)
    }

    Arc::new(RwLock::new(UploadDelegatorsPool {
        delegators: Arc::new(RwLock::new(delegators)),
    }))
}

pub fn get_upload_delegator_pool(max_upload_number: u16) -> Arc<RwLock<UploadDelegatorsPool>> {
    unsafe {
        if UPLOAD_DELEGATORS_POOL.is_none() {
            let pool = init_upload_delegators_pool(max_upload_number);
            UPLOAD_DELEGATORS_POOL.replace(pool);
        }
        UPLOAD_DELEGATORS_POOL.clone().unwrap()
    }
}

impl UploadDelegatorsPool {
    // add code here
    pub fn request_delegator(&self) -> Option<Arc<RwLock<UploadDelegator>>> {
        let mut receivers_arc = self.delegators.write();
        let r = receivers_arc.pop();
        r
    }

    pub fn return_back_delegator(&self, receiver: Arc<RwLock<UploadDelegator>>) {
        let mut receivers_arc = self.delegators.write();
        receivers_arc.push(receiver);
    }
}
