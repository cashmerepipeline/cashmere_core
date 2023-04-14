use async_trait::async_trait;
use data_server::ResumePoint;
use futures::FutureExt;
use log::{debug, info};
use serde::Serialize;
use tokio::sync::mpsc;
use tokio_stream::{StreamExt, wrappers::ReceiverStream};
use tonic::{Response, Status};

use data_server::file_utils::{check_chunk_md5, create_recieve_data_file_stream};
use data_server::UploadDelegator;
use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::manage_ids::*;
use view;

use crate::{RequestStream, ResponseStream, StreamResponseResult};

#[async_trait]
pub trait HandleUploadFile {
    async fn handle_upload_file(
        &self,
        request: RequestStream<UploadFileRequest>,
    ) -> StreamResponseResult<UploadFileResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();
        let role_group = auth::get_current_role(metadata).unwrap();

        let mut in_stream = request.into_inner();
        let first_request = if let Some(in_data) = in_stream.next().await {
            if in_data.is_ok() {
                in_data.unwrap()
            } else {
                return Err(Status::data_loss("请求数据错误"));
            }
        } else {
            return Err(Status::data_loss("数据流错误"));
        };

        let data_id = first_request.data_id.clone();
        let specs = first_request.specs.clone();
        let stage = first_request.stage.clone();
        let version = first_request.version.clone();
        let sub_path = first_request.sub_path.clone();
        let file_info = first_request.file_info.clone().unwrap();
        
        // 检查必填项
        if data_id.is_empty() || specs.is_empty() || stage.is_empty() || version.is_empty() {
            return Err(Status::invalid_argument(t!("必填项为缺失")));
        }

        if !view::can_manage_write(&account_id, &role_group, &DATAS_MANAGE_ID.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        // 交互流
        let (resp_tx, resp_rx) = mpsc::channel(5);

        // 请求上传文件代理
        let data_server_arc = data_server::get_data_server();

        let delegator_arc = if let Some(d) = data_server_arc.get_upload_delegator() {
            d
        } else {
            return Err(Status::aborted(
                "获取文件上传代理失败，请重试上传或者等待3分钟后重试。",
            ));
        };

        let (data_folder, data_file_path) = match delegator_arc.prepare_file_uploading(
            &data_id,
            &specs,
            &stage,
            &version,
            &sub_path,
            &file_info,
            file_info.size,
        ) {
            Ok(r) => r,
            Err(e) => {
                return Err(Status::aborted(format!(
                    "{}-{}",
                    e.details(),
                    e.operation()
                )));
            }
        };

        let data_file = match delegator_arc
            .get_upload_target_file(&data_folder, &data_file_path)
            .await
        {
            Ok(f) => f,
            Err(e) => {
                return Err(Status::aborted(format!(
                    "{}-{}",
                    e.details(),
                    e.operation()
                )));
            }
        };

        // 磁盘空间是否足够
        if !delegator_arc.check_disk_space_enough(file_info.size).await {
            return Err(Status::aborted(format!("磁盘空间不足，无法上传文件。")));
        };

        let ftx = match delegator_arc
            .get_receive_file_stream_sender(data_file, data_file_path.to_str().unwrap().to_string())
            .await
        {
            Ok(s) => s,
            Err(e) => {
                return Err(Status::aborted(format!(
                    "{}-{}",
                    e.operation(),
                    e.details()
                )));
            }
        };

        //  起始数据块编号
        let resume_point = delegator_arc
            .check_and_read_resume_point(&data_file_path)
            .await;

        let (mut next_chunk_index, mut resume_chunk_md5) = if let Ok(r) = resume_point {
            (r.chunk_index+1, r.chunk_md5)
        } else {
            // 读取失败，从1开始
            (1, "".to_string())
        };


        info!("开始接收文件: {}--{}", &data_id, &file_info.file_name);

        if let Ok(_r) = resp_tx
            .send(Ok(UploadFileResponse {
                next_chunk_index: next_chunk_index as u64,
            }))
            .await
        {
            info!("返回起始包编号：{}", &next_chunk_index);
        } else {
            return Err(Status::aborted("返回起始包编号失败。"));
        };

        // 2. 接收线程, 等待客户端发送完成后发出关闭信号后结束退出
        // TODO: 需要设置最大等待时长
        tokio::spawn(async move {
            let file_name = file_info.file_name.clone();
            let data_id = first_request.data_id.clone();
            let mut retry_count = 0u16;

            // 发送到文件写入流
            while let Some(result) = in_stream.next().await {
                match result {
                    Ok(v) => {
                        info!(
                            "接收到数据块：{}-{}-{}-{}",
                            v.data_id,
                            file_name,
                            v.current_chunk_index,
                            v.chunk.len()
                        );

                        if v.current_chunk_index == 0 {
                            info!("文件传输完，开始校验文件");
                            // TODO: 文件校验, 失败
                            // 成功退出
                            break;
                        } else {
                            // 数据块校验, 如果失败则重发
                            if check_chunk_md5(&v.chunk_md5, &v.chunk) {
                                debug!("数据块校验成功，发送到文件写入流。");
                                if let Ok(r) = ftx.send(v.chunk).await {
                                    r
                                } else {
                                    resp_tx
                                        .send(Err(Status::data_loss("发送文件流失败。")))
                                        .await
                                        .expect("反馈错误失败。");
                                    return Err(Status::data_loss("发送文件流失败。"));
                                }
                                // 下一个数据块编号, 如果超过最大值则返回0，标志文件传输完成
                                next_chunk_index = next_chunk_index + 1;
                                resume_chunk_md5 = v.chunk_md5;
                                retry_count = 0;
                            } else {
                                info!("数据块校验失败，重发数据块。");
                                // 不改变数据块编号
                                
                                retry_count = retry_count + 1;
                                // 重试次数超过5次则退出
                                if retry_count > 5 {
                                    resp_tx
                                        .send(Err(Status::data_loss("数据块校验失败，重试次数超过5次。")))
                                        .await
                                        .expect("反馈错误失败。");
                                    return Err(Status::data_loss("数据块校验失败，重试次数超过5次。"));
                                }
                            }

                            if next_chunk_index as u64 > v.total_chunks {
                                debug!("文件传输完，返回0，标志文件传输完成。{}", file_name);
                                next_chunk_index = 0;
                            }

                            if resp_tx
                                .send(Ok(UploadFileResponse {next_chunk_index: next_chunk_index as u64}))
                                .await
                                .is_err()
                            {
                                resp_tx
                                    .send(Err(Status::data_loss("返回下一个包编号错误。")))
                                    .await
                                    .expect("反馈错误失败。");
                                return Err(Status::data_loss("返回下一个数据包编号失败。"));
                            };
                        }

                        ()
                    }
                    Err(_err) => {
                        // 记录续传点
                        delegator_arc
                            .record_resume_point(&data_file_path, ResumePoint {
                                chunk_index: next_chunk_index-1,
                                chunk_md5: resume_chunk_md5,
                            })
                            .await;

                        resp_tx
                            .send(Err(Status::data_loss("接收上传流发生错误。")))
                            .await
                            .expect("反馈错误失败。");
                        return Err(Status::data_loss("接收文件上传错误。"));
                    }
                }
            }

            // 文件上传结束后必须返还代理，否则代理将丢失
            delegator_arc
                .delete_resume_point_file(&data_file_path)
                .await;
            data_server_arc.return_back_upload_delegator(delegator_arc);
            info!("传输文件结束: {}--{}。", data_id, file_name);
            Ok(())
        });

        let resp_stream = ReceiverStream::new(resp_rx);

        Ok(Response::new(
            Box::pin(resp_stream) as ResponseStream<UploadFileResponse>
        ))
    }
}
