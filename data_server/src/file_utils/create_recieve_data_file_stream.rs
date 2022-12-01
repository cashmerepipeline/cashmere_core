use std::path;
use std::path::PathBuf;

use fs4::tokio::AsyncFileExt;
use log::info;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;

use cash_result::{operation_failed, OperationResult};
use manage_define::cashmere::FileInfo;

use crate::file_utils::check_space_enough::check_space_enough;

// 创建文件数据流，返回流发送端
pub async fn create_recieve_data_file_stream(
    data_id: &String,
    file_info: &FileInfo,
) -> Result<Sender<Vec<u8>>, OperationResult> {
    //  会被移动到async块
    // TODO:尝试使用其他方式不需要移动，或者
    let data_id = data_id.clone();

    // 检查文件空间是否足够传参不使用引用
    if check_space_enough(file_info.size).is_err() {
        return Err(operation_failed(
            "create_recieve_data_file_stream",
            "存储空间不足。",
        ));
    }

    // 创建文件，失败则需要提前返回
    let data_configes = configs::get_data_configs();

    let data_folder: PathBuf = [&data_configes.root, &data_id].iter().collect();
    let file_ext = path::Path::new(&file_info.file_name).extension().unwrap();
    let mut file_path_buf: PathBuf = [&data_configes.root, &data_id, &file_info.md5]
        .iter()
        .collect();
    file_path_buf.set_extension(file_ext);

    if fs::create_dir_all(data_folder).await.is_err() {
        return Err(operation_failed(
            "create_recieve_data_file_stream",
            "创建目录失败。",
        ));
    };

    let mut data_file = match File::create(&file_path_buf).await {
        Ok(f) => f,
        Err(_e) => {
            return Err(operation_failed(
                "create_recieve_data_file_stream",
                "创建文件失败。",
            ));
        }
    };

    info!("为数据分配存储空间：{}", &data_id);
    if data_file.allocate(file_info.size).await.is_err() {
        return Err(operation_failed(
            "create_recieve_data_file_stream",
            "分配文件空间失败.",
        ));
    };

    // 使用缓存减少磁盘操作
    // 缓存, 最大为640kb = 1024*128*5，满后写入临时文件,缓存长度是5
    // 每块最大为128kb
    let mut buffer: Vec<Option<Vec<u8>>> = vec![None; 5];
    let mut cursor = 0usize;

    let (ftx, mut frx) = mpsc::channel::<Vec<u8>>(5);

    // 发出线程后不能等待写入完成，否则会卡死，一直等待数据写入
    tokio::spawn(async move {
        while let Some(part) = frx.recv().await {
            buffer[cursor] = Some(part);

            // 写出缓存
            if cursor >= 4 {
                while let Some(bpart) = buffer.iter().next() {
                    if data_file
                        .write_all(bpart.as_deref().unwrap())
                        .await
                        .is_err()
                    {
                        return Err(operation_failed(
                            "create_recieve_data_file_stream",
                            "写入文件错误。",
                        ));
                    };
                }
                // 重置下标
                cursor = 0;
                continue;
            }

            cursor = cursor + 1;
        }

        // 缓存刷出
        for bpart in buffer.iter() {
            if bpart.is_none() {
                break;
            }
            if data_file
                .write_all(bpart.as_deref().unwrap())
                .await
                .is_err()
            {
                return Err(operation_failed(
                    "create_recieve_data_file_stream",
                    "写入文件错误。",
                ));
            };
        }
        info!("完成写入文件:{}", data_id);
        Ok(())
    });

    Ok(ftx)
}
