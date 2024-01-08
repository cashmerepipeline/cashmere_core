use cash_result::{operation_failed, operation_succeed, OperationResult};
use dependencies_sync::{rust_i18n::{self, t}, log};
use manage_define::manage_ids::MANAGES_MANAGE_ID;

pub async fn init_check(manage_id: &str) -> Result<OperationResult, OperationResult> {
    log::info!("{}: {}", t!("管理器数据库检查"), manage_id);

    // 检查数据库是否存在管理集合，不存在则需要创建管理集合
    if !database::collection_exists(manage_id).await {
        return Err(operation_failed(
            "ManagerTrait::init",
            format!(
                "{}: {}",
                t!("数据库管理集合不存在, 请先初始化数据库"),
                manage_id
            ),
        ));
    }

    // 检查管理实体是否存在，不存在给出错误信息
    if entity::exists_by_id(MANAGES_MANAGE_ID, manage_id)
        .await
        .is_none()
    {
        return Err(operation_failed(
            "ManagerTrait::init",
            format!(
                "{}: {}",
                t!("管理实体不存在, 需要先初始化管理实体"),
                manage_id
            ),
        ));
    }

    // 检查序列号生成器
    if let Err(_e) = database::init_ids_count_field(manage_id).await {
        return Err(operation_failed(
            "ManagerTrait::init",
            t!("初始化序列号生成器失败"),
        ));
    }

    log::info!("管理器数据库检查完成：{}", manage_id);
    Ok(operation_succeed("ok"))
}
