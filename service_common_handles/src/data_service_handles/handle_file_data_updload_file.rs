use crate::{RequestStream, ResponseStream, StreamResponseResult};
use async_trait::async_trait;
use log::{debug, info};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::{Response, Status};

use data_utils::file_utils::create_recieve_data_file_stream;
use majordomo::{self, get_majordomo};
use manage_define::cashmere::*;
use manage_define::field_ids::*;
use manage_define::general_field_ids::*;
use manage_define::manage_ids::*;
use managers::traits::ManagerTrait;
use managers::utils::make_new_entity_document;
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

        let mut in_stream = request.into_inner();
        let first_request = if let Some(in_data) = in_stream.next().await{
            if in_data.is_ok() {
                in_data.unwrap()
            } else {
                return Err(Status::data_loss("请求数据错误"))
            }
        } else {
            return Err(Status::data_loss("数据流错误"));
        };

        let data_id = first_request.data_id.clone();
        let file_info = first_request.file_info.clone().unwrap();

        if !view::can_manage_write(&account_id, &groups, &DATAS_MANAGE_ID.to_string()).await {
            return Err(Status::unauthenticated("用户不具有可写权限"));
        }
        // 取得第一个可写组作为组
        let group_id =
            match view::get_first_write_group(&groups, &DATAS_MANAGE_ID.to_string()).await {
                Some(r) => r,
                None => return Err(Status::unauthenticated("用户不具有可写权限")),
            };

        let majordomo_arc = get_majordomo().await;
        let data_manager = majordomo_arc
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
        let mut next_chunk_index = 1u64;
        info!("开始接收文件{}--{}", &data_id, &file_info.file_name);

        //  起始数据块编号
        resp_tx
            .send(Ok(FileDataUploadFileResponse {
                next_chunk_index: next_chunk_index,
            }))
            .await;

        // 2. 接收线程, 等待客户端发送完成后发出关闭信号后结束退出
        // TODO: 需要设置最大等待时长
        let result = tokio::spawn(async move {
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
                            println!("到达末尾");
                            // TODO: 数据块校验, 如果失败则重发
                        } else {
                            if let Ok(r) = ftx.send(v.chunk).await {
                                r
                            } else {
                                ()
                            }

                            // TODO: 下一个数据块编号
                            next_chunk_index = next_chunk_index + 1;
                            if next_chunk_index > v.total_chunks {
                                next_chunk_index = 0;
                            }
                            resp_tx
                                .send(Ok(FileDataUploadFileResponse {
                                    next_chunk_index: next_chunk_index,
                                }))
                                .await;
                        }

                        ()
                    }
                    Err(err) => {
                        // if let Some(io_err) = match_for_io_error(&err) {
                        //     if io_err.kind() == ErrorKind::BrokenPipe {
                        //         // here you can handle special case when client
                        //         // disconnected in unexpected way
                        //         eprintln!("\tclient disconnected: broken pipe");
                        //         break;
                        //     }
                        // }
                    }
                }
            }
            info!("接收文件结束{}--{}", data_id, file_name);
        });

        let resp_stream = ReceiverStream::new(resp_rx);

        Ok(Response::new(
            Box::pin(resp_stream) as ResponseStream<FileDataUploadFileResponse>
        ))
    }
}
