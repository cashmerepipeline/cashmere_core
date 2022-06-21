use std::path;
use std::path::PathBuf;
use cash_result::{operation_failed, OperationResult};
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc;
use tokio::fs::File;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use manage_define::cashmere::FileInfo;
use crate::file_utils::check_space_enough::check_space_enough;

// 创建文件数据流，返回流发送端
pub async fn create_recieve_data_file_stream(
    data_id: &String,
    file_info: &FileInfo,
) -> Result<Sender<Vec<u8>>, OperationResult>{
    // 检查文件空间是否足够
    if check_space_enough(file_info.size).await.is_err() {
       return Err(operation_failed( "create_recieve_data_file_stream", "存储空间不足。"));
    }

    // 创建文件，失败则需要提前返回
    let data_configes = configs::get_data_configs();

    let data_folder:PathBuf = [&data_configes.root, data_id].itor().collect();
    let file_ext = path::Path::new(&file_info.file_name).extension().unwrap();
    let mut file_path_buf:PathBuf = [
        &data_configes.root,
        &data_id,
        &file_info.md5].iter().collect();
    file_path_buf.set_extension(file_ext);

    if fs::create_dir_all(data_folder).await.is_err(){
        return Err(operation_failed( "create_recieve_data_file_stream", "创建目录失败。"));
    };

    let mut f = match File::create(&file_path).await {
        Ok(f) => f,
        Err(e) => {
            return Err(operation_failed( "create_recieve_data_file_stream", "创建文件失败。"));
        }
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
                    if f.write(bpart.unwrap().as_ref()).await.is_err(){
                        return Err(operation_failed( "create_recieve_data_file_stream", "写入文件错误。"));
                    };
                }
                cursor = 0;
            }

            cursor = cursor + 1;
        }

        // 缓存刷出
        while let Some(bpart) = buffer.iter().next() {
            if f.write(bpart.unwrap().as_ref()).await.is_err(){
                return Err(operation_failed( "create_recieve_data_file_stream", "写入文件错误。"));
            };
        }
        Ok(())
    });

    Ok(ftx)
}