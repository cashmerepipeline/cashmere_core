use std::io::Error;

/// 文件接收器
#[derive(Debug)]
pub struct UploadDelegator {}

/// 申请磁盘空间
fn request_storage_space(request_size: u128)->Result<(), Error>{
    Ok(())
}

/// 创建文件
fn create_recieve_target_file(){}

/// 续传检查
fn check_is_continue() {}

