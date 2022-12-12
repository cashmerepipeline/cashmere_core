use serde_derive::Deserialize;

#[derive(Deserialize, Clone)]
pub struct DataServerConfigs {
    // 数据根目录
    pub root_dir_path: String,
    // 文件最大大小
    pub max_file_size: u128,
    // 文件集最大数量
    pub max_set_size: u32,
    // 文件序列最大数量
    pub max_sequence_length: u32,
    // 上传数据块最大数量
    pub transfer_chunk_size: u32,
    // 最大文件上传连接
    pub max_file_upload_number: u16,
    // 最大文件下载连接
    pub max_file_download_number: u16,
}
