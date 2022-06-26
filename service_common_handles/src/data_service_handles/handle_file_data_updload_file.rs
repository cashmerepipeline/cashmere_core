use crate::{RequestStream, UnaryResponseResult};
use async_trait::async_trait;
use bson::{doc, Document};
use chrono::format::parse;
use futures::{StreamExt, TryFutureExt};
use log::{debug, info};
use std::io::ErrorKind;
use std::ops::Deref;
use tonic::{Request, Response, Status, Streaming};

use super::utils::match_for_io_error;
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
    ) -> UnaryResponseResult<FileDataUploadFileResponse> {
        let metadata = request.metadata();
        // 已检查过，不需要再检查正确性
        let token = auth::get_auth_token(metadata).unwrap();
        let (account_id, groups) = auth::get_claims_account_and_roles(&token).unwrap();

        let mut in_stream = request.into_inner();
        let first_request = in_stream
            .next()
            .await
            .expect("数据流错误")
            .expect("流内部错误");

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

        // 1. 创建文件流
        let ftx = if let Ok(r) = create_recieve_data_file_stream(&data_id, &file_info).await {
            r
        } else {
            return Err(Status::aborted("创建文件流错误。"));
        };

        info!("开始接收文件{}--{}", &data_id, &file_info.file_name);

        // 2. 接收线程, 等待客户端发送完成后发出关闭信号后结束退出
        // TODO: 需要设置最大等待时长
        let result = tokio::spawn(async move {
            let file_name = file_info.file_name.clone();
            let data_id = first_request.data_id.clone();

            // 发送到文件写入流
            ftx.send(first_request.chunk).await.unwrap();
            while let Some(result) = in_stream.next().await {
                match result {
                    Ok(v) => {
                        debug!(
                            "接收到数据{}{}{}",
                            v.data_id,
                            v.current_chunk_index,
                            v.chunk.len()
                        );

                        // TODO: 数据块校验, 如果失败则重发

                        if let Ok(r) = ftx.send(v.chunk).await {
                            r
                        } else {
                            ()
                        }

                        // TODO: 下一个数据块编号
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
        })
        .await;

        // 5. 返回结果
        match result {
            Ok(_r) => Ok(Response::new(FileDataUploadFileResponse {
                result: data_id,
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                "创建文件流失败",
                e.to_string() // "{} {}", "创建文件流失败", "error",
            ))),
        }
    }
}
