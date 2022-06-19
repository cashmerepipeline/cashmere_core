use std::error::Error;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tonic::Status;
use crate::cashmere::FileInfo;

pub fn match_for_io_error(err_status: &Status) -> Option<&std::io::Error> {
    let mut err: &(dyn Error + 'static) = err_status;

    loop {
        if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
            return Some(io_err);
        }

        // h2::Error do not expose std::io::Error with `source()`
        // https://github.com/hyperium/h2/pull/462
        if let Some(h2_err) = err.downcast_ref::<h2::Error>() {
            if let Some(io_err) = h2_err.get_io() {
                return Some(io_err);
            }
        }

        err = match err.source() {
            Some(err) => err,
            None => return None,
        };
    }
}

/// 缓存, 最大为640kb = 1024*128*5，满后写入临时文件,缓存长度是5
/// 每块最大为128kb
pub async fn create_recieve_data_file_stream(
    data_id: &String,
    file_info: &FileInfo,
) -> Sender<Vec<u8>> {
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
