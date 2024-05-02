use dependencies_sync::bson::{self, Document};
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::Status;

use managers::ManagerTrait;

/// zh: 验证单个key的上传数据，在验证管理编号之后
pub async fn validate_data_field(manage_id: &str, data: &Vec<u8>) -> Result<(), Status> {
    let d = if let Ok(d) = bson::from_slice::<Document>(data) {
        d
    } else {
        return Err(Status::invalid_argument(format!(
            "{}: {}",
            t!("无效数据"),
            manage_id
        )));
    };

    let manager = {
        let major = majordomo::get_majordomo();
        major.get_manager_by_id(manage_id).unwrap()
    };

    //  zh: 检查是否为空， 只有一个key
    let ks: Vec<i32> = d.keys().map(|x| x.parse::<i32>().unwrap()).collect();
    if ks.is_empty() || ks.len() > 1 {
        return Err(Status::invalid_argument(format!(
            "{}: {}",
            t!("无效数据"),
            manage_id,
        )));
    }

    // 检查是否在描写中
    let schema = manager.get_manage_schema().await;
    if schema.iter().map(|x| x.id).any(|x| x == ks[0]) {
        Ok(())
    } else {
        Err(Status::invalid_argument(format!(
            "{}: {}, {}",
            t!("无效数据, 描写中不存在"),
            manage_id,
            ks[0]
        )))
    }
}
