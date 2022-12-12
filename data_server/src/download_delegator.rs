use std::{path::{Path, PathBuf}, sync::Arc, fmt::format};

use configs::DataServerConfigs;
use log::{info};

use fs4::tokio::AsyncFileExt;
use parking_lot::RwLock;
use tokio::{fs, sync::mpsc::Receiver, io::AsyncReadExt};
use tokio::fs::File;
use tokio::io::AsyncBufRead;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;

use cash_result::{Failed, operation_failed, OperationResult};
use manage_define::cashmere::FileInfo;

use crate::{file_utils::check_space_enough, upload_delegators_pool::get_upload_delegator_pool};

#[derive(Debug)]
/// 上传代理
pub struct DownloadDelegator{
    pub transfer_chunk_size: usize,
}

impl DownloadDelegator {
    pub fn new(data_server_configs: DataServerConfigs)->DownloadDelegator{
        DownloadDelegator { transfer_chunk_size: data_server_configs.transfer_chunk_size as usize }
    }

    pub async fn check_request_file_exists(
        &self,
        data_id: &String,
        data_file_path: &String
    ) ->Result<PathBuf, OperationResult>{
        let data_root = &configs::get_data_server_configs().root_dir_path;

        let mut file_pathbuf = PathBuf::new();
        file_pathbuf.push(data_root);
        file_pathbuf.push(data_id);
        file_pathbuf.push(data_file_path);

        if file_pathbuf.exists(){
            Ok(file_pathbuf)
        } else{
            Err(OperationResult::Failed(Failed{
                operation: "check_request_file_exists".to_string(),
                details: "文件不存在".to_string(),
            }))
        }
    }

    /// 创建文件发送流
    /// 返回流接收端
    pub async fn create_send_file_stream(
        &'static self,
        data_file_path: PathBuf,
    ) -> Result<Receiver<Vec<u8>>, OperationResult> {
        // 使用缓存减少磁盘操作
        // 缓存, 最大为640kb = 1024*128*5，满后写入临时文件,缓存长度是5
        let file_path_str = String::from(data_file_path.to_str().unwrap());
        let mut data_file = match File::open(data_file_path).await{
            Ok(f) => f,
            Err(e) => return Err(OperationResult::Failed(Failed{
                operation: "create_send_file_stream".to_string(),
                details: format!("打开文件失败: {}", file_path_str),
            }))
        };

        let mut buffer = bytes::BytesMut::with_capacity(self.transfer_chunk_size*5);

        let (ftx, mut frx) = mpsc::channel::<Vec<u8>>(5);

        // 发出线程后不能等待写入完成，否则会卡死，一直等待数据写入
        tokio::spawn(async move {
            while let Ok(_n) = data_file.read_buf(&mut buffer).await {
                while let Some(chunk) = buffer.chunks(self.transfer_chunk_size).next(){
                    match ftx.send(chunk.to_vec()).await{
                        Err(e)=> log::error!("发送数据块失败: {}", file_path_str),
                        _ => ()
                    }
                };
                buffer.clear()
            }
            
            info!("发送文件完成{}", file_path_str);
        });

        Ok(frx)
    }
    
}
