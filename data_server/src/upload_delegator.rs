use std::{path::{Path, PathBuf}, sync::Arc};

use log::info;

use fs4::tokio::AsyncFileExt;
use parking_lot::RwLock;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;

use cash_result::{Failed, operation_failed, OperationResult};
use configs::get_data_server_configs;
use manage_define::cashmere::FileInfo;

use crate::{file_utils::check_space_enough, upload_delegators_pool::get_upload_delegator_pool};

#[derive(Debug)]
/// 上传代理
pub struct UploadDelegator;

impl UploadDelegator {
    /// 申请磁盘空间
    /// 成功返回 （上传文件目录，上传文件路径）
    pub fn check_storage_space(
        &self,
        data_id: &String,
        file_info: &FileInfo,
        request_size: u64,
    ) -> Result<(PathBuf, PathBuf), OperationResult> {
        let data_root = &get_data_server_configs().root_dir_path;

        let mut data_dir_path = PathBuf::new();
        data_dir_path.push(data_root);
        data_dir_path.push(data_id);

        match check_space_enough(data_dir_path.as_path(), request_size) {
            Ok(_r) => {
                let file_ext = Path::new(&file_info.file_name).extension().unwrap();
                let mut file_path_buf: PathBuf = [data_dir_path.to_str().unwrap(), &file_info.md5]
                    .iter()
                    .collect();
                file_path_buf.set_extension(file_ext);
                Ok((data_dir_path.to_path_buf(), file_path_buf))
            }
            Err(e) => Err(OperationResult::Failed(Failed {
                operation: "check_space_enough".to_string(),

                details: format!(
                    "{}, 存储空间不足: {}",
                    e,
                    data_dir_path.to_str().unwrap(),
                ),
            })),
        }
    }

    /// 创建文件
    pub async fn create_upload_target_file(
        &self,
        target_pathes: &(PathBuf, PathBuf),
    ) -> Result<File, OperationResult> {
        let (data_folder, data_file_path) = target_pathes;

        if fs::create_dir_all(data_folder).await.is_err() {
            return Err(operation_failed(
                "create_recieve_data_file_stream",
                "创建目录失败。",
            ));
        };

        let data_file = match File::create(&data_file_path).await {
            Ok(f) => f,
            Err(_e) => {
                return Err(operation_failed(
                    "create_recieve_data_file_stream",
                    "创建文件失败。",
                ));
            }
        };

        Ok(data_file)
    }

    /// 分配空间
    pub async fn allocate_space(
        &self,
        data_file: File,
        file_size: u64,
    ) -> Result<File, OperationResult> {
        if data_file.allocate(file_size).await.is_err() {
            return Err(operation_failed(
                "create_recieve_data_file_stream",
                "分配文件空间失败.",
            ));
        }

        Ok(data_file)
    }

    pub async fn create_receive_file_stream(
        &self,
        mut data_file: File,
        data_file_path: String,
    ) -> Result<Sender<Vec<u8>>, OperationResult> {
        // 使用缓存减少磁盘操作
        // 缓存, 最大为640kb = 1024*128*5，满后写入临时文件,缓存长度是5
        // 每块最大为128kb
        let mut buffer: Vec<Option<Vec<u8>>> = vec![None; 5];
        let mut cursor = 0usize;

        let (ftx, mut frx) = mpsc::channel::<Vec<u8>>(5);

        // 发出线程后不能等待写入完成，否则会卡死，一直等待数据写入
        tokio::spawn(async move {
            while let Some(part) = frx.recv().await {
                buffer[cursor] = Some(part);

                // 写出缓存
                if cursor >= 4 {
                    while let Some(bpart) = buffer.iter().next() {
                        if data_file
                            .write_all(bpart.as_deref().unwrap())
                            .await
                            .is_err()
                        {
                            return Err(operation_failed(
                                "create_recieve_data_file_stream",
                                "写入文件错误。",
                            ));
                        };
                    }
                    // 重置下标
                    cursor = 0;
                    continue;
                }

                cursor = cursor + 1;
            }

            // 缓存刷出
            for bpart in buffer.iter() {
                if bpart.is_none() {
                    break;
                }
                if data_file
                    .write_all(bpart.as_deref().unwrap())
                    .await
                    .is_err()
                {
                    return Err(operation_failed(
                        "create_recieve_data_file_stream",
                        "写入文件错误。",
                    ));
                };
            }
            info!("完成写入文件:{}", data_file_path);
            Ok(())
        });

        Ok(ftx)
    }

    /// 续传检查
    pub fn check_is_continue(&self) {}
    
}
