use std::ops::Deref;
use async_trait::async_trait;
use bson::{doc, Document};
use chrono::format::parse;
use futures::{StreamExt, TryFutureExt};
use tonic::{Request, Response, Status, Streaming};

use crate::cashmere::*;
use crate::{RequestStream, UnaryResponseResult};

use majordomo::{self, get_majordomo};
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
        let first_request = in_stream.next().await
            .expect("数据流错误")
            .expect("流内部错误");

        let data_id = &first_request.data_id.clone();
        let md5 = &first_request.md5.clone();
        let total_chuncks = &first_request.total_chuncks.clone();
        let current_chunck = &first_request.current_chunck.clone();
        let chunck = &first_request.chunck.clone();
        let file_name = &first_request.file_name.clone();
        let last_modified = &first_request.last_modified.clone();

        let name = if name.is_some() {
            name.as_ref().unwrap()
        } else {
            return Err(Status::data_loss("名字必须提供"));
        };

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

        // 1. 创建文件信息
        let file_info = FileInfo {
            file_name: String::from(file_name),
            md5: String::from(md5),
            size: total_chuncks * 128 * 1024,
            last_modified: *last_modified,
        };

        // 3. 开始接收文件，并以流的形式写入文件
        // 缓存, 最大为500kb = 1024*10*50，满后写入临时文件,缓存长度是50
        // 每块最大为10kb
        let mut last_request = None;
        let mut wav_data: Vec<UploadRequest> = vec![first_request];
        let mut init_length = 0u8;
        while let Some(part) = in_stream.next().await {
            let part = part.unwrap();
            last_request = Some(part.clone());
            wave_data.push(part);
            if wave_data.len() > 50 {
                // 写入临时文件, 发送到流

                // 清空缓存
                wav_data.clear();
            }
        }
        // 4. 完成后，更新文件信息
        // 5. 返回文件写入结果
        match result {
            Ok(_r) => Ok(Response::new(FileDataUploadFileResponse {
                result: data_id.unwrap().clone(),
            })),
            Err(e) => Err(Status::aborted(format!(
                "{} {}",
                e.operation(),
                e.details()
            ))),
        }
    }
}