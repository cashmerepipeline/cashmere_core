use manage_define::cashmere::FileInfo;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use cash_result::{Failed, operation_failed, OperationResult};
use tokio::fs::{File, OpenOptions};
use log::{debug, info};
use tokio::fs;
use tokio::sync::mpsc::Sender;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::sync::mpsc;
use bytes::BufMut;
use crate::file_utils::check_space_enough;
use crate::ResumePoint;

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
        specs: &String,
        stage: &String,
        version: &String,
        sub_path: &String,
        file_info: &FileInfo,
        request_size: u64,
    ) -> Result<(PathBuf, PathBuf), OperationResult> {
        let data_root = &configs::get_data_server_configs().root_dir_path;

        let mut data_dir_path = PathBuf::new();
        data_dir_path.push(data_root);
        data_dir_path.push(data_id);
        data_dir_path.push(specs);
        data_dir_path.push(stage);
        data_dir_path.push(version);
        data_dir_path.push(sub_path);

        match check_space_enough(data_dir_path.as_path(), request_size) {
            Ok(_r) => {
                let file_ext = Path::new(&file_info.file_name).extension().unwrap();
                let file_name = Path::new(&file_info.file_name).file_name().unwrap();
                let mut file_pathbuf = PathBuf::from(data_dir_path.clone());
                file_pathbuf.push(file_name);
                file_pathbuf.set_extension(file_ext);

                Ok((data_dir_path.to_path_buf(), file_pathbuf))
            }

            Err(e) => Err(OperationResult::Failed(Failed {
                operation: "check_space_enough".to_string(),

                details: format!("{}, 存储空间不足: {}", e, data_dir_path.to_str().unwrap(),),
            })),
        }
    }

    /// 取得上传目标文件
    pub async fn get_upload_target_file(
        &self,
        data_folder: &PathBuf,
        data_file_path: &PathBuf,
    ) -> Result<File, OperationResult> {
        debug!("开始创建文件:{}", data_file_path.to_str().unwrap());

        if fs::create_dir_all(data_folder).await.is_err() {
            return Err(operation_failed(
                "get_upload_target_file",
                format!("创建目录失败: {}", data_folder.to_str().unwrap()),
            ));
        };

        // 如果文件已存在，则以追加的方式打开
        if data_file_path.exists() {
            debug!(
                "文件已存在:{}, 以追加方式打开文件",
                data_file_path.to_str().unwrap()
            );
            let data_file = match OpenOptions::new().append(true).open(&data_file_path).await {
                Ok(f) => return Ok(f),
                Err(_e) => {
                    return Err(operation_failed(
                        "get_upload_target_file",
                        format!("打开文件失败: {}", data_file_path.to_str().unwrap()),
                    ));
                }
            };
            return Err(operation_failed(
                "get_upload_target_file",
                format!("文件已存在: {}", data_file_path.to_str().unwrap()),
            ));
        } else {
            // 如果文件不存在，则创建文件
            let data_file = match File::create(&data_file_path).await {
                Ok(f) => f,
                Err(_e) => {
                    return Err(operation_failed(
                        "get_upload_target_file",
                        format!("创建文件失败: {}", data_file_path.to_str().unwrap()),
                    ));
                }
            };

            debug!("创建文件成功:{}", data_file_path.to_str().unwrap());

            return Ok(data_file);
        }
    }

    /// 检查磁盘空间是否足够
    pub async fn check_disk_space_enough(&self, file_size: u64) -> bool {
        let data_root = &configs::get_data_server_configs().root_dir_path;
        let data_root_path = Path::new(data_root);

        let available_space = match fs4::available_space(data_root_path) {
            Ok(m) => m,
            Err(_e) => {
                return false;
            }
        };

        if available_space < file_size {
            return false;
        }

        true
    }

    /// 检查并读取续传点记录文件，如果存在，则取得续传的位置
    pub async fn check_and_read_resume_point(
        &self,
        data_file_path: &PathBuf,
    ) -> Result<ResumePoint, OperationResult> {
        let mut resume_point_file_path = data_file_path.clone();
        resume_point_file_path.set_extension("resume");

        if resume_point_file_path.exists() {
            let mut resume_point_file = match File::open(&resume_point_file_path).await {
                Ok(f) => f,
                Err(_e) => {
                    return Err(operation_failed(
                        "check_and_read_resume_point",
                        format!(
                            "打开续传点记录文件失败: {}",
                            resume_point_file_path.to_str().unwrap()
                        ),
                    ));
                }
            };

            let mut resume_content = String::new();
            let resume_point = match resume_point_file.read_to_string(&mut resume_content).await {
                Ok(_r) => {
                    match ResumePoint::from_str(&resume_content) {
                        Ok(r) => r,
                        Err(_e) => {
                            return Err(operation_failed(
                                "check_and_read_resume_point",
                                format!(
                                    "解析续传点记录文件失败: {}",
                                    resume_point_file_path.to_str().unwrap()
                                ),
                            ));
                        }
                    }
                },
                Err(_e) => {
                    return Err(operation_failed(
                        "check_and_read_resume_point",
                        format!(
                            "读取续传点记录文件失败: {}",
                            resume_point_file_path.to_str().unwrap()
                        ),
                    ));
                }
            };

            debug!(
                "读取续传点记录文件成功:{}",
                resume_point_file_path.to_str().unwrap()
            );

            Ok(resume_point)
        } else {
            Err(operation_failed(
                "check_and_read_resume_point",
                format!(
                    "续传点记录文件不存在: {}",
                    resume_point_file_path.to_str().unwrap()
                ),
            ))
        }
    }

    /// 创建续传点文件，并将续传点写入文件
    /// 记录的师最后一次正确传输的数据块的编号和md5值
    pub async fn record_resume_point(
        &self,
        data_file_path: &PathBuf,
        resume_point: ResumePoint,
    ) -> Result<(), OperationResult> {
        let mut resume_point_file_path = data_file_path.clone();
        resume_point_file_path.set_extension("resume");

        let mut resume_point_file = match File::create(&resume_point_file_path).await {
            Ok(f) => f,
            Err(_e) => {
                return Err(operation_failed(
                    "record_resume_point",
                    format!(
                        "创建续传点记录文件失败: {}",
                        resume_point_file_path.to_str().unwrap()
                    ),
                ));
            }
        };

        let resume_str = toml::to_string(&resume_point).unwrap();
        if resume_point_file
            .write_all(resume_str.as_ref())
            .await
            .is_err()
        {
            return Err(operation_failed(
                "record_resume_point",
                format!(
                    "写入续传点记录文件失败: {}",
                    resume_point_file_path.to_str().unwrap()
                ),
            ));
        }

        debug!(
            "写入续传点记录文件成功:{}",
            resume_point_file_path.to_str().unwrap()
        );

        Ok(())
    }

    /// 删除续传点文件
    pub async fn delete_resume_point_file(
        &self,
        data_file_path: &PathBuf,
    ) -> Result<(), OperationResult> {
        let mut resume_point_file_path = data_file_path.clone();
        resume_point_file_path.set_extension("resume");

        if resume_point_file_path.exists() {
            if fs::remove_file(&resume_point_file_path).await.is_err() {
                return Err(operation_failed(
                    "delete_resume_point_file",
                    format!(
                        "删除续传点记录文件失败: {}",
                        resume_point_file_path.to_str().unwrap()
                    ),
                ));
            }

            debug!(
                "删除续传点记录文件成功:{}",
                resume_point_file_path.to_str().unwrap()
            );
        }

        Ok(())
    }

    pub async fn get_receive_file_stream_sender(
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
            // 刷出文件，关闭缓存
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
}
