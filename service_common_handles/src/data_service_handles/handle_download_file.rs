use async_trait::async_trait;
use data_server::UploadDelegator;
use data_server::file_utils::create_recieve_data_file_stream;
use futures::FutureExt;
use log::info;
use serde::Serialize;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{Response, Status};

use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::manage_ids::*;
use view;

use crate::{RequestStream, ResponseStream, StreamResponseResult};

#[async_trait]
pub trait HandleDownloadFile {
    async fn handle_download_file(
        &self,
        request: RequestStream<DownloadFileRequest>,
    ) -> StreamResponseResult<DownloadFileResponse> {
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
        let file_path = first_request.file_info.clone().unwrap();

        if !view::can_manage_write(&account_id, &role_group, &DATAS_MANAGE_ID.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        // 交互流
        let (resp_tx, resp_rx) = mpsc::channel(5);

        // 请求上传文件代理
        let data_server_arc = data_server::get_data_server();

        let delegator = if let Some(d) = data_server_arc.get_upload_delegator(&file_info) {
            d
        } else {
            return Err(Status::aborted(
                "获取文件上传代理失败，请重试上传或者等待3分钟后重试。",
            ));
        };

        let data_pathes =
            match delegator.check_storage_space(&data_id, &file_info, file_info.size) {
                Ok(r) => r,
                Err(e) => {
                    return Err(Status::aborted(format!(
                        "{}-{}",
                        e.details(),
                        e.operation()
                    )));
                }
            };

        let data_file = match delegator.create_upload_target_file(&data_pathes).await {
            Ok(f) => f,
            Err(e) => {
                return Err(Status::aborted(format!(
                    "{}-{}",
                    e.details(),
                    e.operation()
                )));
            }
        };

        let mut data_file = match delegator.allocate_space(data_file, file_info.size).await {
            Ok(f) => f,
            Err(e) => {
                return Err(Status::aborted(format!(
                    "{}-{}",
                    e.details(),
                    e.operation()
                )))
            }
        };

        let ftx = match delegator
            .create_receive_file_stream(data_file, data_pathes.1.to_str().unwrap().to_string())
            .await
        {
            Ok(s) => s,
            Err(e) => {
                return Err(Status::aborted(format!(
                    "{}-{}",
                    e.operation(),
                    e.details()
                )))
            }
        };

        // // 创建文件流
        // let ftx = if let Ok(r) = create_recieve_data_file_stream(&data_id, &file_info).await {
        //     r
        // } else {
        //     return Err(Status::aborted("创建文件流错误。"));
        // };

        // TODO: 续传检查
        //  起始数据块编号
        let mut next_chunk_index = 1u64;
        info!("开始接收文件{}--{}", &data_id, &file_info.file_name);

        if let Ok(_r) = resp_tx
            .send(Ok(FileDataUploadFileResponse {
                next_chunk_index: next_chunk_index,
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

            // 发送到文件写入流, 第一个包没有数据，丢弃
            while let Some(result) = in_stream.next().await {
                match result {
                    Ok(v) => {
                        info!(
                            "接收到数据{}-{}-{}",
                            v.data_id,
                            v.current_chunk_index,
                            v.chunk.len()
                        );

                        if v.current_chunk_index == 0 {
                            info!("文件传输完，开始校验文件");
                            // TODO: 文件校验, 失败
                        } else {
                            // TODO: 数据块校验, 如果失败则重发
                            if let Ok(r) = ftx.send(v.chunk).await {
                                r
                            } else {
                                resp_tx
                                    .send(Err(Status::data_loss("发送文件流失败。")))
                                    .await
                                    .expect("反馈错误失败。");
                                return Err(Status::data_loss("发送文件流失败。"));
                            }

                            // TODO: 下一个数据块编号
                            next_chunk_index = next_chunk_index + 1;
                            if next_chunk_index > v.total_chunks {
                                next_chunk_index = 0;
                            }
                            if resp_tx
                                .send(Ok(FileDataUploadFileResponse { next_chunk_index }))
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
                        resp_tx
                            .send(Err(Status::data_loss("接收上传流发生错误。")))
                            .await
                            .expect("反馈错误失败。");
                        return Err(Status::data_loss("接收文件上传错误。"));
                    }
                }
            }

            // 文件上传结束后必须返还代理，否则将丢失代理
            data_server_arc.return_back_delegator(delegator);
            info!("接收文件结束{}--{}。", data_id, file_name);
            Ok(())
        });

        let resp_stream = ReceiverStream::new(resp_rx);

        Ok(Response::new(
            Box::pin(resp_stream) as ResponseStream<DownloadFileResponse>
        ))
    }
}

