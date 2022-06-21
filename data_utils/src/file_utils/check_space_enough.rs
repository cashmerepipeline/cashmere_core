use std::path::Path;
use fs4::available_space;
use configs::get_data_configs;


/// 检查存储空间是否足够
pub async fn check_space_enough(
    file_size: u64,
) -> Result<(), String> {
    let data_root = get_data_configs().root;
    let data_path = Path::new(data_root.as_ref());

    let space = available_space(data_path).await;
    if space < file_size {
        return Err(format!("空间不足{}", file_size));
    }

    Ok(())
}