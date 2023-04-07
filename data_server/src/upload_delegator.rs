use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use bytes::BufMut;
use log::{debug, info};

use fs4::tokio::AsyncFileExt;
use parking_lot::RwLock;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::{fs, io::BufWriter};

use cash_result::{operation_failed, Failed, OperationResult};
use manage_define::cashmere::FileInfo;

use crate::{file_utils::check_space_enough, upload_delegators_pool::get_upload_delegator_pool};

#[derive(Debug)]
/// 上传代理
pub struct UploadDelegator {
    pub transfer_chunk_size: usize,
}

impl UploadDelegator {
    /// 准备文件上传
    /// 成功返回 （上传文件目录，上传文件路径）
    pub fn prepare_file_uploading(
        &self,
        data_id: &String,
        stage: &String,
        version: &String,
        file_info: &FileInfo,
        request_size: u64,
    ) -> Result<(PathBuf, PathBuf), OperationResult> {
        let data_root = &configs::get_data_server_configs().root_dir_path;

        let mut data_dir_path = PathBuf::new();
        data_dir_path.push(data_root);
        data_dir_path.push(data_id);
        data_dir_path.push(stage);
        data_dir_path.push(version);

        match check_space_enough(data_dir_path.as_path(), request_size) {
            Ok(_r) => {
                let file_ext = Path::new(&file_info.file_name).extension().unwrap();
                let file_name = Path::new(&file_info.file_name).file_name().unwrap();
                let mut file_pathbuf = PathBuf::from(data_dir_path.clone());
                file_pathbuf.push(file_name);
                file_pathbuf.set_extension(file_ext);

                // 如果文件已存在，则返回错误
                if file_pathbuf.exists() {
                    log::error!("文件已存在: {}", file_pathbuf.to_str().unwrap());

                    return Err(operation_failed("check_space_enough", "文件已存在。"));
                }

                Ok((data_dir_path.to_path_buf(), file_pathbuf))
            }
            Err(e) => Err(OperationResult::Failed(Failed {
                operation: "check_space_enough".to_string(),

                details: format!("{}, 存储空间不足: {}", e, data_dir_path.to_str().unwrap(),),
            })),
        }
    }

    /// 创建文件
    pub async fn create_upload_target_file(
        &self,
        target_pathes: &(PathBuf, PathBuf),
    ) -> Result<File, OperationResult> {
        let (data_folder, data_file_path) = target_pathes;

        debug!("开始创建文件:{}", data_file_path.to_str().unwrap());

        if fs::create_dir_all(data_folder).await.is_err() {
            return Err(operation_failed(
                "create_recieve_data_file_stream",
                format!("创建目录失败: {}", data_folder.to_str().unwrap()),
            ));
        };

        let data_file = match File::create(&data_file_path).await {
            Ok(f) => f,
            Err(_e) => {
                return Err(operation_failed(
                    "create_upload_target_file",
                    format!("创建文件失败: {}", data_file_path.to_str().unwrap()),
                ));
            }
        };

        debug!("创建文件成功:{}", data_file_path.to_str().unwrap());

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

        debug!("分配空间成功:{}", file_size);
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
        let capacity = 5;
        let chunk_size = self.transfer_chunk_size;

        let mut buffer = bytes::BytesMut::with_capacity(capacity * self.transfer_chunk_size);
        let mut writer = BufWriter::new(data_file);

        debug!("创建文件写入流:{}", data_file_path);
        let (ftx, mut frx) = mpsc::channel::<Vec<u8>>(5);

        let mut total_size = 0u64;
        // 发出线程后不能等待写入完成，否则会卡死，一直等待数据写入
        tokio::spawn(async move {
            while let Some(part) = frx.recv().await {
                debug!("文件流接收到数据块到缓存");
                total_size = total_size + part.len() as u64;

                // 缓存未满，写入缓存
                if buffer.len() < capacity * chunk_size {
                    buffer.put(&part[..]);
                    continue;
                } else {
                    // 缓存已满，写入文件
                    if writer.write_all(&buffer).await.is_err() {
                        return Err(operation_failed(
                            "create_recieve_data_file_stream",
                            "流数据写入文件错误。",
                        ));
                    };

                    // 重置缓存
                    buffer.clear();
                    buffer.put(&part[..]);
                }
            }

            // 剩余数据写入文件
            if writer.write_all(&buffer).await.is_err() {
                return Err(operation_failed(
                    "create_recieve_data_file_stream",
                    "流数据写入文件错误。",
                ));
            };
            // 刷出缓存
            if writer.flush().await.is_err() {
                return Err(operation_failed(
                    "create_recieve_data_file_stream",
                    "流数据写入文件错误。",
                ));
            };

            info!("数据流完成写入文件: {}-{}", data_file_path, total_size);
            Ok(())
        });

        Ok(ftx)
    }

    /// 续传检查, 返回续传块的起始编号
    pub async fn check_continue(&self, data_file_path: &PathBuf) -> u64 {
        // TODO: 检查起始位置
        return 1u64;
    }
}
