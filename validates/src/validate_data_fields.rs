use dependencies_sync::bson::{self, Document};
use dependencies_sync::rust_i18n::{self, t};
use dependencies_sync::tonic::Status;

use managers::ManagerTrait;

/// zh: 验证单个key的上传数据，在验证管理编号之后
pub async fn validate_data_fields(manage_id: &str, data: &[u8]) -> Result<(), Status> {
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

    //  zh: 检查是否为空
    let keys: Vec<i32> = d.keys().map(|x| x.parse::<i32>().unwrap()).collect();
    if keys.is_empty() {
        return Err(Status::invalid_argument(format!(
            "{}: {}",
            t!("无效数据, 数据为空"),
            manage_id,
        )));
    }

    // 检查是否在描写中
    let schema = manager.get_manage_schema().await;
    for key in keys.iter() {
        if schema.iter().map(|x| x.id).any(|x| &x == key) {
            continue;
        } else {
            return Err(Status::invalid_argument(format!(
                "{}: {}, {}",
                t!("无效数据, 描写中不存在"),
                manage_id,
                key
            )));
        }
    }

    Ok(())
}
