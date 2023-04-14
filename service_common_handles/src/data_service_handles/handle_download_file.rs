use async_trait::async_trait;
use data_server::file_utils::{create_recieve_data_file_stream, get_chunk_md5};
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
        let stage = first_request.stage.clone();
        let version = first_request.version.clone();
        let chunk_index = first_request.chunk_index.clone();
        let file_name = first_request.file_name.clone();

        if !view::can_manage_read(&account_id, &role_group, &DATAS_MANAGE_ID.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }

        // 交互流
        let (resp_tx, resp_rx) = mpsc::channel(5);

        // 请求上传文件代理
        let data_server_arc = data_server::get_data_server();

        let delegator = if let Some(d) = data_server_arc.get_download_delegator() {
            d
        } else {
            return Err(Status::aborted(
                "获取文件下载代理失败，请重试或者等待3分钟后重试。",
            ));
        };

        let file_path = match delegator
            .check_request_file_exists(&data_id, &stage, &version, &file_name)
            .await
        {
            Ok(r) => r,
            Err(e) => {
                return Err(Status::aborted(format!(
                    "{}-{}",
                    e.details(),
                    e.operation()
                )));
            }
        };

        // 绑定文件流
        let (ftx, mut frx) = mpsc::channel(5);
        match delegator.bind_file_stream_sender(file_path, ftx).await {
            Ok(_s) => _s,
            Err(e) => {
                return Err(Status::aborted(format!(
                    "{}-{}",
                    e.operation(),
                    e.details()
                )))
            }
        };

        // TODO: 续传检查
        //  起始数据块编号
        let mut current_chunk_index = 0u64;

        info!(
            "开始发送文件{}--{}--{}--{}",
            &data_id, &stage, &version, &file_name
        );

        tokio::spawn(async move {
            let file_name = file_name.clone();
            let data_id = first_request.data_id.clone();

            let mut current_chunk = vec![];

            // TODO: 添加文件信息
            // 第一个包只包含文件信息，不包含数据
            let first_resp = DownloadFileResponse {
                data_id: data_id.clone(),
                chunk_index: 0,
                chunk_md5: get_chunk_md5(&current_chunk),
                chunk: vec![],
            };
            resp_tx.send(Ok(first_resp.clone())).await;

            while let Some(request) = in_stream.next().await {
                match request {
                    Ok(r) => {
                        // 如果请求编号为0，表示请求有错误，发送结束标识
                        if r.chunk_index == 0 {
                            current_chunk = vec![];
                            current_chunk_index = 0;
                        } else {
                            // 检查数据块编号,如果不是当前编号，发送新数据块
                            if r.chunk_index != current_chunk_index {
                                if let Some(chunk) = frx.recv().await {
                                    current_chunk = chunk;
                                    current_chunk_index = r.chunk_index;
                                } else {
                                    // 到达文件末尾，发送结束标记
                                    current_chunk = vec![];
                                    current_chunk_index = 0;
                                }
                                info!(
                                    "发送数据块{}-{}-{}",
                                    data_id,
                                    current_chunk_index,
                                    current_chunk.len()
                                );
                            } else {
                                // 重发数据块
                                info!(
                                    "重新发送数据块{}-{}-{}",
                                    data_id,
                                    current_chunk_index,
                                    &current_chunk.len()
                                );
                            }
                        }

                        let current_resp = DownloadFileResponse {
                            data_id: data_id.clone(),
                            chunk_index: current_chunk_index.clone(),
                            chunk_md5: get_chunk_md5(&current_chunk),
                            chunk: current_chunk.clone(),
                        };

                        if resp_tx.send(Ok(current_resp.clone())).await.is_err() {
                            resp_tx
                                .send(Err(Status::data_loss("发送下一个块错误。")))
                                .await
                                .expect("反馈错误失败。");

                            return Err(Status::data_loss(format!(
                                "发送数据包失败。{}",
                                current_chunk_index
                            )));
                        };
                    }
                    Err(e) => {
                        return Err(Status::data_loss(format!("下载请求错误。{}", e)));
                    }
                }
            }

            // 文件上传结束后必须返还代理，否则将丢失代理
            data_server_arc.return_back_download_delegator(delegator);
            info!("发送文件结束{}--{}。", data_id, file_name);
            Ok(())
        });

        let resp_stream = ReceiverStream::new(resp_rx);

        Ok(Response::new(
            Box::pin(resp_stream) as ResponseStream<DownloadFileResponse>
        ))
    }
}
