use std::sync::Arc;

use dependencies_sync::log::info;
use dependencies_sync::rust_i18n::{self, t};

use crate::majordomo::Majordomo;

static mut MAJORDOMO: Option<Arc<Majordomo>> = None;

/// 从设置新建管理管理器
fn init_majordomo() -> Arc<Majordomo> {
    info!("{}", t!("初始化主管实例"));

    let majordomo = Majordomo::new();
    Arc::new(majordomo)
}

pub fn get_majordomo() -> Arc<Majordomo> {
    unsafe {
        // 有数据
        if MAJORDOMO.is_some() {
            MAJORDOMO.clone().unwrap()
        }
        // 没有则新解空表
        else {
            MAJORDOMO.replace(init_majordomo());
            MAJORDOMO.clone().unwrap()
        }
    }
}