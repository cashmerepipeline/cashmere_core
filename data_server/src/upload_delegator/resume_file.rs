use crate::file_utils::check_space_enough;
use crate::ResumePoint;
use bytes::BufMut;
use cash_result::{operation_failed, Failed, OperationResult};
use log::{debug, info};
use manage_define::cashmere::FileInfo;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;

use super::UploadDelegator;

impl UploadDelegator {
    /// 检查续传点文件是否存在
    pub async fn check_resume_point_file_exists(&self, data_file_path: PathBuf) -> bool {
        let mut resume_point_file_path = data_file_path.clone();
        resume_point_file_path.set_extension("resume");
        if resume_point_file_path.exists() {
            return true;
        }
        false
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
                Ok(_r) => match ResumePoint::from_str(&resume_content) {
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
}
