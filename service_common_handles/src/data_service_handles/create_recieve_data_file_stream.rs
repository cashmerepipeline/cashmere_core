use std::path;
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use crate::cashmere::FileInfo;

/// 缓存, 最大为640kb = 1024*128*5，满后写入临时文件,缓存长度是5
/// 每块最大为128kb
pub async fn create_recieve_data_file_stream(
    data_id: &String,
    file_info: &FileInfo,
) -> Sender<Vec<u8>> {
    // 检查文件空间是否足够
    // 创建文件，失败则需要提前返回
    let data_configes = configs::get_data_configs();
    let file_path = path::join(
        &data_configes.root,
        &data_id,
        &file_info.md5,
    );
    let mut f = match File::create(&file_path).await {
        Ok(f) => f,
        Err(e) => {
            println!("创建文件失败{}", e);
        }
    };

    //
    let mut buffer: Vec<Option<Vec<u8>>> = vec![None; 5];
    let mut cursor = 0usize;

    let (ftx, mut frx) = mpsc::channel::<Vec<u8>>(5);
    let result = tokio::spawn(async move {
        while let Some(part) = frx.recv().await {
            println!("{}", part.len());
            buffer[cursor] = Some(part);

            // 写出缓存
            if cursor >= 4 {
                while let Some(bpart) = buffer.iter().next() {
                    println!("write buffer into file");
                }
                cursor = 0;
            }

            cursor = cursor + 1;
        }
        // 缓存刷出
        for bpart in buffer.iter() {
            if let Some(bpart) = bpart {
                println!("write buffer into file {}", bpart.len());
            }
        }
    });

    ftx
}
