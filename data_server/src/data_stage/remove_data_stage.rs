use std::path::{PathBuf, Path};

use cash_result::{OperationResult, operation_failed};
use tokio::fs::{self, create_dir_all};
use tokio::fs::File;

use super::DataStage;

/// 只删除阶段软连接文件
pub async fn remove_data_stage(
    data_root_dir: &String,
    stage: &String,
) -> Result<(), OperationResult> {
    let mut data_root_path = PathBuf::new();
    data_root_path.push(data_root_dir);

    let mut stage_path = PathBuf::new();
    stage_path.push(data_root_dir);
    stage_path.push(stage);

    // 空文件，用于占位
    if stage_path.exists(){
        if fs::remove_file(stage_path.clone()).await.is_err(){
            return Err(operation_failed("remove_data_stage", t!("删除数据阶段失败")));
        };
    }

    Ok(())
}

