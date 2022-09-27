use crate::{RequestStream, ResponseStream, StreamResponseResult};
use async_trait::async_trait;
use log::info;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{Response, Status};

use data_utils::file_utils::create_recieve_data_file_stream;
use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::manage_ids::*;
use view;

#[async_trait]
pub trait HandleFileDataUploadFile {
    async fn handle_file_data_upload_file(
        &self,
        request: RequestStream<FileDataUploadFileRequest>,
    ) -> StreamResponseResult<FileDataUploadFileResponse> {
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
        let file_info = first_request.file_info.clone().unwrap();

        if !view::can_manage_write(&account_id, &role_group, &DATAS_MANAGE_ID.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }
        // 取得第一个可写组作为组
        let _group_id =
            match view::get_first_write_group(&groups, &DATAS_MANAGE_ID.to_string()).await {
                Some(r) => r,
                None => return Err(Status::unauthenticated("用户不具有可写权限")),
            };

        let majordomo_arc = get_majordomo().await;
        let _data_manager = majordomo_arc
            .get_manager_by_id(DATAS_MANAGE_ID)
            .await
            .unwrap();

        // 交互流
        let (resp_tx, resp_rx) = mpsc::channel(5);

        //  创建文件流
        let ftx = if let Ok(r) = create_recieve_data_file_stream(&data_id, &file_info).await {
            r
        } else {
            return Err(Status::aborted("创建文件流错误。"));
        };

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
            info!("接收文件结束{}--{}。", data_id, file_name);
            Ok(())
        });

        let resp_stream = ReceiverStream::new(resp_rx);

        Ok(Response::new(
            Box::pin(resp_stream) as ResponseStream<FileDataUploadFileResponse>
        ))
    }
}
