use async_trait::async_trait;
use data_server::file_utils::{create_recieve_data_file_stream, get_chunk_md5};
use data_server::UploadDelegator;
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

        // 文件流
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
        let mut next_chunk_index = 1u64;
        info!(
            "开始发送文件{}--{}--{}--{}",
            &data_id, &stage, &version, &file_name
        );

        tokio::spawn(async move {
            let file_name = file_name.clone();
            let data_id = first_request.data_id.clone();

            // 发送到文件写入流, 第一个包没有数据，丢弃
            while let Some(chunk) = frx.recv().await {
                info!("发送数据块{}-{}-{}", data_id, next_chunk_index, chunk.len());

                let resp = DownloadFileResponse {
                    data_id: data_id.clone(),
                    chunk_index: next_chunk_index,
                    chunk_md5: get_chunk_md5(&chunk),
                    chunk: chunk,
                };

                if resp_tx.send(Ok(resp)).await.is_err() {
                    resp_tx
                        .send(Err(Status::data_loss("返回下一个包编号错误。")))
                        .await
                        .expect("反馈错误失败。");

                    return Err(Status::data_loss(format!("发送数据包失败。{}", next_chunk_index)));
                };
            }

            // 文件上传结束后必须返还代理，否则将丢失代理
            data_server_arc.return_back_download_delegator(delegator);
            info!("接收文件结束{}--{}。", data_id, file_name);
            Ok(())
        });

        let resp_stream = ReceiverStream::new(resp_rx);

        Ok(Response::new(
            Box::pin(resp_stream) as ResponseStream<DownloadFileResponse>
        ))
    }
}
