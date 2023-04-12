use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use configs::DataServerConfigs;
use log::{info, debug};

use fs4::tokio::AsyncFileExt;
use parking_lot::RwLock;
use tokio::fs::File;
use tokio::io::AsyncBufRead;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::{fs, io::AsyncReadExt, sync::mpsc::Receiver};

use cash_result::{operation_failed, Failed, OperationResult};
use manage_define::cashmere::FileInfo;

use crate::{file_utils::check_space_enough, upload_delegators_pool::get_upload_delegator_pool};

#[derive(Debug)]
/// 上传代理
pub struct DownloadDelegator;

impl DownloadDelegator {
    pub async fn check_request_file_exists(
        &self,
        data_id: &String,
        stage: &String,
        version: &String,
        file_name: &String,
    ) -> Result<PathBuf, OperationResult> {
        let data_root = &configs::get_data_server_configs().root_dir_path;

        let mut file_pathbuf = PathBuf::new();
        file_pathbuf.push(data_root);
        file_pathbuf.push(data_id);
        file_pathbuf.push(stage);
        file_pathbuf.push(version);
        file_pathbuf.push(file_name);

        if file_pathbuf.exists() {
            Ok(file_pathbuf)
        } else {
            Err(OperationResult::Failed(Failed {
                operation: "check_request_file_exists".to_string(),
                details: "文件不存在".to_string(),
            }))
        }
    }

    /// 绑定文件发送流
    pub async fn bind_file_stream_sender(
        &self,
        data_file_path: PathBuf,
        ftx: Sender<Vec<u8>>,
    ) -> Result<(), OperationResult> {
        let file_path_str = String::from(data_file_path.to_str().unwrap());
        let mut data_file = match File::open(data_file_path).await {
            Ok(f) => f,
            Err(e) => {
                return Err(OperationResult::Failed(Failed {
                    operation: "create_send_file_stream".to_string(),
                    details: format!("打开文件失败: {}", file_path_str),
                }))
            }
        };
        
        let chunk_size = configs::get_data_server_configs().transfer_chunk_size as usize;
        // 缓存容量, 为 chunk_size*5，满后写入临时文件,缓存长度是5
        let capacity = chunk_size * 5;
        info!("缓存大小为: {}, 块大小为: {}", capacity, chunk_size);

        tokio::spawn(async move {
            let mut buffer = bytes::BytesMut::with_capacity(capacity.clone());
            let meta_size = data_file.metadata().await.unwrap().len();
            let mut total_size = 0u64;

            while let Ok(n) = data_file.read_buf(&mut buffer).await {
                info!("读取到数据块:{:?}, {}, {}", file_path_str, buffer.len(), n);
                // 读取到文件末尾, 退出
                if n == 0 {
                    break;
                }

                for chunk in buffer.chunks(chunk_size).next() {
                    match ftx.send(chunk.to_vec()).await {
                        Err(e) => log::error!("发送数据块失败: {}", file_path_str),
                        _ => (),
                    }
                }
                total_size += n as u64;
                buffer.clear()
            }

            info!("发送文件完成{}-{}-{}", file_path_str, total_size, meta_size);
        });
        
        Ok(())
    }
}
