use std::path::PathBuf;
use dependencies_sync::tokio::fs;
use cash_result::OperationResult;

/// 使用dependencies_sync::tokio异步删除指定目录下和指定列表中的文件夹和文件
pub async fn delete_version_folder_entries(
    version_path: &PathBuf,
    entries: &Vec<String>,
) -> Result<(), OperationResult> {
    for entry in entries {
        let entry_path = version_path.join(entry);
        if entry_path.is_dir() {
            fs::remove_dir_all(entry_path).await;
        } else {
            fs::remove_file(entry_path).await;
        }
    }
    Ok(())
}