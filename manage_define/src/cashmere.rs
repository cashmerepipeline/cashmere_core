/// 名
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Name {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
/// 名属性，语言：语言名
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NameField {
    #[prost(map="string, string", tag="1")]
    pub name_field: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// 重命名
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameRequest {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    #[prost(string, tag="2")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub new_name: ::core::option::Option<Name>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 新语言名
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewLanguageNameRequest {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    #[prost(string, tag="2")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub new_name: ::core::option::Option<Name>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewLanguageNameResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 移除语言名
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLanguageNameRequest {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    #[prost(string, tag="2")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub language: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLanguageNameResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 新品牌
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewCountryRequest {
    #[prost(message, optional, tag="1")]
    pub name: ::core::option::Option<Name>,
    #[prost(string, tag="2")]
    pub native: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub code: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub phone_area_code: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="5")]
    pub languages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewCountryResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LanguageCode {
    #[prost(string, tag="1")]
    pub code: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub name_map: ::prost::alloc::vec::Vec<u8>,
}
/// 新语言编码
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewLanguageCodeRequest {
    #[prost(message, optional, tag="1")]
    pub name: ::core::option::Option<Name>,
    #[prost(string, tag="2")]
    pub code: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub native_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewLanguageCodeResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// TODO: 可能不需要
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditLanguageCodeRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub new_code: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub new_native: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditLanguageCodeResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewGroupRequest {
    #[prost(message, optional, tag="1")]
    pub name: ::core::option::Option<Name>,
    #[prost(string, tag="2")]
    pub new_group_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewGroupResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int32Range {
    #[prost(int32, tag="1")]
    pub min: i32,
    #[prost(int32, tag="2")]
    pub max: i32,
    #[prost(int32, tag="3")]
    pub value: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int64Range {
    #[prost(int64, tag="1")]
    pub min: i64,
    #[prost(int64, tag="2")]
    pub max: i64,
    #[prost(int64, tag="3")]
    pub value: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UInt32Range {
    #[prost(uint32, tag="1")]
    pub min: u32,
    #[prost(uint32, tag="2")]
    pub max: u32,
    #[prost(uint32, tag="3")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloatRange {
    #[prost(float, tag="1")]
    pub min: f32,
    #[prost(float, tag="2")]
    pub max: f32,
    #[prost(float, tag="3")]
    pub value: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoubleRange {
    #[prost(double, tag="1")]
    pub min: f64,
    #[prost(double, tag="2")]
    pub max: f64,
    #[prost(double, tag="3")]
    pub value: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Color {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub red: u32,
    #[prost(uint32, tag="3")]
    pub green: u32,
    #[prost(uint32, tag="4")]
    pub blue: u32,
    #[prost(uint32, tag="5")]
    pub alpha: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataInfo {
    #[prost(enumeration="DataType", tag="1")]
    pub data_type: i32,
    #[prost(int32, tag="3")]
    pub owner_manage_id: i32,
    #[prost(string, tag="4")]
    pub owner_entity_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="5")]
    pub stages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// 新建数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewDataRequest {
    #[prost(message, optional, tag="1")]
    pub name: ::core::option::Option<Name>,
    #[prost(enumeration="DataType", tag="2")]
    pub data_type: i32,
    #[prost(int32, tag="3")]
    pub owner_manage_id: i32,
    #[prost(string, tag="4")]
    pub owner_entity_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewDataResponse {
    /// 成功返回 "ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 取得数据信息
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataInfoRequest {
    #[prost(string, tag="1")]
    pub data_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataInfoResponse {
    #[prost(message, optional, tag="1")]
    pub data_info: ::core::option::Option<DataInfo>,
}
/// 删除数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkDataRemovedRequest {
    #[prost(int32, tag="1")]
    pub owner_manage_id: i32,
    #[prost(string, tag="2")]
    pub owner_entity_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub data_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkDataRemovedResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 取得实体数据表
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntityDataRequest {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    #[prost(string, tag="2")]
    pub entity_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntityDataResponse {
    #[prost(string, repeated, tag="1")]
    pub data_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DataType {
    /// 单个文件
    FileData = 0,
    /// 序列文件
    SequenceData = 1,
    /// 多类型文件集合
    FileSetData = 2,
    /// 类json格式数据
    DocumentData = 3,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataServerConfigs {
    #[prost(string, tag="1")]
    pub root_dir_path: ::prost::alloc::string::String,
    /// 文件最大大小, 16MB
    #[prost(uint64, tag="2")]
    pub max_file_size: u64,
    /// 文件集最大数量, 1000
    #[prost(uint32, tag="3")]
    pub max_set_size: u32,
    /// 文件序列最大数量
    #[prost(uint64, tag="4")]
    pub max_sequence_length: u64,
    /// 最大文件上传连接
    #[prost(uint32, tag="5")]
    pub max_file_upload_number: u32,
    /// 最大文件下载连接
    #[prost(uint32, tag="6")]
    pub max_file_download_number: u32,
    /// 块最大大小，1024*128=128KB
    #[prost(uint32, tag="7")]
    pub transfer_chunk_size: u32,
    /// 内部文件路径，不需要通过服务器上传文件, 可将文件直接存储到目标位置
    /// {"windows"="X:/data_root/dir", "linux"="/mnt/data_root/dir", "macos" = "/mnt/data_root/dir"}
    #[prost(map="string, string", tag="8")]
    pub internal_root_dir_map: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// 取得数据服务设置
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataServerConfigsRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataServerConfigsResponse {
    #[prost(message, optional, tag="1")]
    pub configs: ::core::option::Option<DataServerConfigs>,
}
/// 数据阶段信息
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataStageInfo {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// stage目录下的文件列表, 文件为文件列表，集合为集合目录列表
    #[prost(bytes="vec", repeated, tag="2")]
    pub versions: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// 当前连接所指版本
    #[prost(string, tag="3")]
    pub current_version: ::prost::alloc::string::String,
}
/// 新阶段
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewDataStageRequest {
    #[prost(string, tag="1")]
    pub data_id: ::prost::alloc::string::String,
    /// 这里可能因为软件对路径字符集支持的不同只能使用英文作为文件名，比如Maya
    #[prost(string, tag="2")]
    pub stage_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewDataStageResponse {
    /// 成功返回 "ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 删除阶段，只删除阶段->数据的文件连接，不删除数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveDataStageRequest {
    #[prost(string, tag="1")]
    pub data_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub stage: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveDataStageResponse {
    /// 成功返回 "ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 取得数据阶段表
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataStagesRequest {
    #[prost(string, tag="1")]
    pub data_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataStagesResponse {
    #[prost(message, repeated, tag="1")]
    pub stages: ::prost::alloc::vec::Vec<DataStageInfo>,
}
/// 添加数据版本到阶段
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddDataStageVersionRequest {
    #[prost(string, tag="1")]
    pub data_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub stage_name: ::prost::alloc::string::String,
    /// 版本一般有具体的含义，不只是一个数字，比如"v001", 数据的名应该与版本一致
    #[prost(string, tag="3")]
    pub version: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddDataStageVersionResponse {
    /// 成功返回 "ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 取得数据阶段版本表
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataStageVersionsRequest {
    #[prost(string, tag="1")]
    pub data_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub stage_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataStageVersionsResponse {
    #[prost(bytes="vec", repeated, tag="1")]
    pub versions: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 改变阶段文件连接
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDataStageCurrentVersionRequest {
    #[prost(string, tag="1")]
    pub data_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub stage_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub target_version: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDataStageCurrentVersionResponse {
    /// 成功返回 "ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 添加数据版本到阶段
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveDataStageVersionRequest {
    #[prost(string, tag="1")]
    pub data_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub stage_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub version: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveDataStageVersionResponse {
    /// 成功返回 "ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 文件信息
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileInfo {
    #[prost(string, tag="1")]
    pub file_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub md5: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub size: u64,
    #[prost(int64, tag="4")]
    pub last_modified_time: i64,
}
/// 上传文件数据
/// 第一个包块编号为0，最后一个包块编号为0, 即从0开始，到0结束
/// 第一个包和最后一个包不包含文件数据，作为传输标记用
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadFileRequest {
    #[prost(string, tag="1")]
    pub data_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub total_chunks: u64,
    #[prost(uint64, tag="3")]
    pub current_chunk_index: u64,
    #[prost(bytes="vec", tag="4")]
    pub chunk: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="6")]
    pub chunk_md5: ::prost::alloc::string::String,
    #[prost(message, optional, tag="5")]
    pub file_info: ::core::option::Option<FileInfo>,
    #[prost(string, tag="8")]
    pub stage: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub version: ::prost::alloc::string::String,
}
/// 下一个包块编号
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadFileResponse {
    #[prost(uint64, tag="1")]
    pub next_chunk_index: u64,
}
/// 下载文件数据
/// 编号为0请求返回文件信息
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadFileRequest {
    #[prost(string, tag="1")]
    pub data_id: ::prost::alloc::string::String,
    /// 相对数据存储根目录
    #[prost(string, tag="2")]
    pub stage: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub chunk_index: u64,
    /// 如果给出版本，则下载对应版本的文件，没有则下载阶段软连接指向的文件
    #[prost(string, tag="4")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub file_name: ::prost::alloc::string::String,
}
/// 返回文件流
/// 最后一个包编号为0, 表示下载完成
/// 最后一个包不包含文件数据，作为传输标记用
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadFileResponse {
    #[prost(string, tag="1")]
    pub data_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub chunk_index: u64,
    #[prost(bytes="vec", tag="4")]
    pub chunk: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="5")]
    pub chunk_md5: ::prost::alloc::string::String,
}
/// 序列数据信息
/// 文件名格式：prefix_name.pattern.type_suffix
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SequenceDataInfo {
    #[prost(string, tag="1")]
    pub prefix_name: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub sequence_pattern: u64,
    #[prost(uint64, tag="3")]
    pub type_suffix: u64,
    #[prost(uint64, tag="4")]
    pub start_index: u64,
    #[prost(uint64, tag="5")]
    pub end_index: u64,
    #[prost(uint64, tag="6")]
    pub total_size: u64,
    #[prost(string, tag="7")]
    pub md5: ::prost::alloc::string::String,
}
/// 上传文件序列
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SequenceDataUploadSequenceRequest {
    #[prost(string, tag="1")]
    pub sequence_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub serial_pattern: ::prost::alloc::string::String,
    #[prost(uint32, tag="3")]
    pub sequence_length: u32,
    #[prost(string, tag="4")]
    pub current_file: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub total_chancks: u64,
    #[prost(uint64, tag="6")]
    pub current_chunck: u64,
    #[prost(bytes="vec", tag="7")]
    pub chunck: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub data_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SequenceDataUploadSequenceResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 下载文件序列
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SequenceDataDownloadSequenceRequest {
    #[prost(string, tag="1")]
    pub data_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SequenceDataDownloadSequenceResponse {
    #[prost(string, tag="8")]
    pub data_id: ::prost::alloc::string::String,
    #[prost(string, tag="1")]
    pub sequence_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub serial_pattern: ::prost::alloc::string::String,
    #[prost(uint32, tag="3")]
    pub sequence_length: u32,
    #[prost(string, tag="4")]
    pub current_file: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub total_chancks: u64,
    #[prost(uint64, tag="6")]
    pub current_chunck: u64,
    #[prost(bytes="vec", tag="7")]
    pub chunck: ::prost::alloc::vec::Vec<u8>,
}
/// 集合数据信息
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDataInfo {
    #[prost(string, tag="1")]
    pub data_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub total_size: u64,
    #[prost(message, repeated, tag="3")]
    pub file_infos: ::prost::alloc::vec::Vec<FileInfo>,
}
/// 取得数据集合信息
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSetDataInfoRequest {
    #[prost(string, tag="1")]
    pub data_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSetDataInfoResponse {
    #[prost(message, optional, tag="1")]
    pub data_info: ::core::option::Option<SetDataInfo>,
}
/// 上传单个文件到集合
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDataUploadFileRequest {
    #[prost(string, tag="1")]
    pub data_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub total_chunks: u64,
    #[prost(uint64, tag="3")]
    pub current_chunk_index: u64,
    #[prost(bytes="vec", tag="4")]
    pub chunk: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="5")]
    pub file_info: ::core::option::Option<FileInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDataUploadFileResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
///上传多个文件到集合, 批量上传可能不是很需要
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDataUploadFilesRequest {
    #[prost(string, tag="1")]
    pub data_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub current_total_chunks: u64,
    #[prost(uint64, tag="3")]
    pub current_chunk_index: u64,
    #[prost(bytes="vec", tag="4")]
    pub chunk: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="5")]
    pub file_info: ::prost::alloc::vec::Vec<FileInfo>,
    #[prost(string, tag="6")]
    pub current_md5: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDataUploadFilesResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 下载文件集合
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDataDownloadSetRequest {
    #[prost(string, tag="1")]
    pub data_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDataDownloadSetResponse {
    #[prost(string, tag="1")]
    pub data_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub set_name: ::prost::alloc::string::String,
    #[prost(uint32, tag="3")]
    pub file_counts: u32,
    #[prost(string, tag="4")]
    pub current_file: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub total_chancks: u64,
    #[prost(uint32, tag="6")]
    pub current_chunck: u32,
    #[prost(bytes="vec", tag="7")]
    pub chunck: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAreaRequest {
    #[prost(message, optional, tag="1")]
    pub name: ::core::option::Option<Name>,
    #[prost(string, tag="2")]
    pub parent_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub code: ::prost::alloc::string::String,
    #[prost(enumeration="AreaLevel", tag="4")]
    pub level: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAreaResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditAreaRequest {
    #[prost(string, tag="1")]
    pub area_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub new_parent_id: ::prost::alloc::string::String,
    #[prost(enumeration="AreaLevel", tag="4")]
    pub new_level: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditAreaResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AreaLevel {
    Country = 0,
    Province = 1,
    City = 2,
    Area = 3,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entity {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub name: ::core::option::Option<Name>,
    #[prost(string, tag="3")]
    pub creator_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub create_timestamp: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub modifier_id: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub modify_timestamp: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub owner_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="8")]
    pub groups: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="9")]
    pub data_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="10")]
    pub comment_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag="11")]
    pub removed: bool,
    #[prost(string, repeated, tag="12")]
    pub removed_data_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// 变更物主
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeOwnerRequest {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    #[prost(string, tag="2")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub old_owner_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub new_owner_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeOwnerResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 太通用，不建议开放
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEntityRequest {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEntityResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 不建议开放
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityRequest {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    #[prost(string, tag="2")]
    pub entity_id: ::prost::alloc::string::String,
    /// {field_id:value, ...}
    #[prost(bytes="vec", tag="3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 通用修改实体属性
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityFieldRequest {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    #[prost(string, tag="2")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub field_id: ::prost::alloc::string::String,
    /// {field_id:value}
    #[prost(bytes="vec", tag="4")]
    pub new_value: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityFieldResponse {
    /// 成功返回新值
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 通用修改MAP实体属性
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityMapFieldRequest {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    #[prost(string, tag="2")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub field_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub key: ::prost::alloc::string::String,
    /// {key:value}
    #[prost(bytes="vec", tag="5")]
    pub new_value: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityMapFieldResponse {
    /// 成功返回新值
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 通用修改MAP移除key
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityMapFieldRemoveKeyRequest {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    #[prost(string, tag="2")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub field_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityMapFieldRemoveKeyResponse {
    /// 成功返回key
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 通用修改List实体属性, 添加成员
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityArrayFieldAddItemsRequest {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    #[prost(string, tag="2")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub field_id: ::prost::alloc::string::String,
    /// {field_id:\[items\]}
    #[prost(bytes="vec", tag="4")]
    pub items: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityArrayFieldAddItemsResponse {
    /// 成功返回"ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 通用修改List实体属性, 移除物体
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityArrayFieldRemoveItemsRequest {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    #[prost(string, tag="2")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub field_id: ::prost::alloc::string::String,
    /// {field_id:\[items\]}
    #[prost(bytes="vec", tag="4")]
    pub items: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityArrayFieldRemoveItemsResponse {
    /// 成功返回"ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 取得实体
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEntityRequest {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    #[prost(string, tag="2")]
    pub entity_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEntityResponse {
    #[prost(bytes="vec", tag="1")]
    pub entity: ::prost::alloc::vec::Vec<u8>,
}
/// 取得多个实体
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEntitiesRequest {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    #[prost(string, repeated, tag="2")]
    pub entity_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEntitiesResponse {
    #[prost(bytes="vec", repeated, tag="1")]
    pub entities: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 取得实体页
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEntitiesPageRequest {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    #[prost(uint32, tag="2")]
    pub page_index: u32,
    #[prost(bytes="vec", tag="3")]
    pub conditions: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEntitiesPageResponse {
    #[prost(bytes="vec", repeated, tag="1")]
    pub entities: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 标记实体已移除
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkEntityRemovedRequest {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    #[prost(string, tag="2")]
    pub entity_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkEntityRemovedResponse {
    /// 成功返回"ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 恢复标记为已移除的实体
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecoverRemovedEntityRequest {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    #[prost(string, tag="2")]
    pub entity_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecoverRemovedEntityResponse {
    /// 成功返回"ok"
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 取得已删除实体页
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRemovedEntitiesPageRequest {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    #[prost(uint32, tag="2")]
    pub page_index: u32,
    #[prost(bytes="vec", tag="3")]
    pub conditions: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRemovedEntitiesPageResponse {
    #[prost(bytes="vec", repeated, tag="1")]
    pub entities: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 取得实体已标记移除数据表
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRemovedDataListRequest {
    #[prost(string, tag="1")]
    pub manage_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub data_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRemovedDataListResponse {
    #[prost(string, repeated, tag="1")]
    pub data_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Manage {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    #[prost(bytes="vec", tag="2")]
    pub name_map: ::prost::alloc::vec::Vec<u8>,
}
/// 取得管理列表
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManagesRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManagesResponse {
    #[prost(message, repeated, tag="1")]
    pub manages: ::prost::alloc::vec::Vec<Manage>,
}
/// 取得记录数量
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManageEntryCountRequest {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManageEntryCountResponse {
    #[prost(uint64, tag="1")]
    pub count: u64,
}
/// 映像请求
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManageViewRequest {
    #[prost(string, tag="1")]
    pub manage_name: ::prost::alloc::string::String,
}
/// 映像返回
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManageViewResponse {
    #[prost(string, tag="1")]
    pub view_token: ::prost::alloc::string::String,
}
/// 管理权限
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeManageReadRuleRequest {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    #[prost(string, tag="2")]
    pub group_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub read_rule: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeManageReadRuleResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeManageWriteRuleRequest {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    #[prost(string, tag="2")]
    pub group_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub write_rule: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeManageWriteRuleResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 集合权限 
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeCollectionReadRuleRequest {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    #[prost(string, tag="2")]
    pub group_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub read_rule: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeCollectionReadRuleResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeCollectionWriteRuleRequest {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    #[prost(string, tag="2")]
    pub group_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub write_rule: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeCollectionWriteRuleResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 描写字段权限 
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeFieldReadRuleRequest {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    #[prost(string, tag="2")]
    pub group_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub field_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub read_rule: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeFieldReadRuleResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeFieldWriteRuleRequest {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    #[prost(string, tag="2")]
    pub group_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub field_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub write_rule: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeFieldWriteRuleResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SchemaField {
    #[prost(int32, tag="1")]
    pub id: i32,
    #[prost(bytes="vec", tag="2")]
    pub name_map: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub data_type: ::prost::alloc::string::String,
    #[prost(bool, tag="4")]
    pub removed: bool,
}
/// 取得管理描写
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManageSchemaRequest {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManageSchemaResponse {
    #[prost(message, repeated, tag="1")]
    pub fields: ::prost::alloc::vec::Vec<SchemaField>,
}
/// 添加管理属性
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSchemaFieldRequest {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    #[prost(message, optional, tag="2")]
    pub field: ::core::option::Option<SchemaField>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSchemaFieldResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 标记属性移除
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkSchemaFieldRemovedRequest {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    #[prost(int32, tag="2")]
    pub field_id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkSchemaFieldRemovedResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 编辑属性名
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditSchemaFieldNameRequest {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    #[prost(int32, tag="2")]
    pub field_id: i32,
    #[prost(string, tag="3")]
    pub language: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub new_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditSchemaFieldNameResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(int32, tag="1")]
    pub event_id: i32,
    #[prost(bytes="vec", tag="2")]
    pub context: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventQueue {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventHandle {
    #[prost(int32, tag="1")]
    pub event_id: i32,
    #[prost(int32, tag="2")]
    pub queue_id: i32,
    #[prost(int32, tag="3")]
    pub manage_id: i32,
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
}
/// 新建事件
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEventRequest {
    #[prost(int32, tag="2")]
    pub manage_id: i32,
    #[prost(string, tag="1")]
    pub event_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEventResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 编辑 事件表
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEventRequest {
    #[prost(int32, tag="1")]
    pub event_id: i32,
    #[prost(bytes="vec", tag="2")]
    pub new_values: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEventResponse {
    #[prost(bool, tag="1")]
    pub failed: bool,
}
/// 添加目标队列
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddEventTargetQueueRequest {
    #[prost(int32, tag="1")]
    pub event_id: i32,
    #[prost(message, optional, tag="3")]
    pub target: ::core::option::Option<EventQueue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddEventTargetQueueResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 新建事件队列
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEventQueueRequest {
    #[prost(int32, tag="2")]
    pub manage_id: i32,
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEventQueueResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 新建事件处理
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEventHandleRequest {
    #[prost(int32, tag="2")]
    pub event_id: i32,
    #[prost(int32, tag="3")]
    pub queue_id: i32,
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEventHandleResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 触发事件
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmitEventRequest {
    #[prost(message, optional, tag="1")]
    pub event: ::core::option::Option<Event>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmitEventResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 连接事件队列
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkEventQueueRequest {
    #[prost(int32, tag="1")]
    pub queue_id: i32,
}
/// 新工作
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewWorkRequest {
    #[prost(string, tag="1")]
    pub manage_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewWorkResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 标记工作状态
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkWorkStatusRequest {
    #[prost(string, tag="1")]
    pub work_id: ::prost::alloc::string::String,
    #[prost(enumeration="WorkStatus", tag="2")]
    pub status: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkWorkStatusResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 工作状态
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WorkStatus {
    WaitingStart = 0,
    Suspending = 1,
    Finished = 2,
    Canceled = 4,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewTaskRequest {
    #[prost(string, tag="1")]
    pub work_node_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub name: ::core::option::Option<Name>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewTaskResponse {
    /// id if success
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewTaskDataRequest {
    #[prost(string, tag="1")]
    pub task_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub data_name: ::core::option::Option<Name>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewTaskDataResponse {
    /// id if success
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssociateDataToTaskRequest {
    #[prost(string, tag="1")]
    pub data_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub task_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssociateDataToTaskResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitTaskRequest {
    #[prost(string, tag="1")]
    pub task_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitTaskResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkTaskStatusRequest {
    #[prost(string, tag="1")]
    pub task_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub status_set_id: ::prost::alloc::string::String,
    #[prost(int32, tag="3")]
    pub status_index: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkTaskStatusResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 新阶段
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewPhaseRequest {
    #[prost(string, tag="1")]
    pub procedure_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub name: ::core::option::Option<Name>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewPhaseResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 标记阶段状态
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkPhaseStatusRequest {
    #[prost(string, tag="1")]
    pub phase_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub status: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkPhaseStatusResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 阶段状态
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PhaseStatus {
    PhaseRunning = 0,
    PhaseSuspending = 1,
    PhaseCanceled = 2,
    PhaseEnd = 3,
}
/// 新工作节点
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewWorkNodeRequest {
    #[prost(string, tag="1")]
    pub phase_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub name: ::core::option::Option<Name>,
    #[prost(string, tag="3")]
    pub template_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewWorkNodeResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 工作流关系
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkNodeLinkRequest {
    #[prost(string, tag="1")]
    pub phase_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub start_node_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub out_slot: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub end_node_id: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub in_slot: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkNodeLinkResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveWorkNodeLinkRequest {
    #[prost(string, tag="1")]
    pub phase_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub start_node_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub out_slot: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub end_node_id: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub in_slot: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveWorkNodeLinkResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 指派工作任务
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssignWorkNodeRequest {
    #[prost(string, tag="1")]
    pub work_node_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub worker_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssignWorkNodeResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 数据依赖和产出
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewDataSlotForWorkNodeRequest {
    #[prost(string, tag="1")]
    pub node_id: ::prost::alloc::string::String,
    #[prost(enumeration="SlotType", tag="2")]
    pub slot_type: i32,
    #[prost(string, tag="3")]
    pub slot_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewDataSlotForWorkNodeResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveDataSlotForWorkNodeRequest {
    #[prost(string, tag="1")]
    pub node_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub slot_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveDataSlotForWorkNodeResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddDataToDataSlotRequest {
    #[prost(string, tag="1")]
    pub node_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub slot_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub data_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddDataToDataSlotResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SlotType {
    /// 参考数据
    RefrenceData = 0,
    /// 依赖
    DepedentData = 1,
    /// 工作输出
    WorkData = 2,
    /// 输出
    OutData = 3,
}
/// 新过程
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewProcedureRequest {
    #[prost(message, optional, tag="1")]
    pub name: ::core::option::Option<Name>,
    #[prost(string, tag="2")]
    pub template_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewProcedureResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewTagTypeRequest {
    #[prost(string, tag="1")]
    pub domain_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub language: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewTagTypeResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddDataFieldToTagTypeRequest {
    #[prost(string, tag="1")]
    pub tag_type_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub language: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub field_name: ::prost::alloc::string::String,
    #[prost(enumeration="TagDataType", tag="4")]
    pub data_type: i32,
    #[prost(bytes="vec", tag="5")]
    pub default_value: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddDataFieldToTagTypeResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveDataFieldFromTagTypeRequest {
    #[prost(string, tag="1")]
    pub tag_type_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub language: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub field_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveDataFieldFromTagTypeResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditDataFieldForTagTypeRequest {
    #[prost(string, tag="1")]
    pub tag_type_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub language: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub old_field_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub new_field_name: ::prost::alloc::string::String,
    #[prost(enumeration="TagDataType", tag="5")]
    pub new_data_type: i32,
    #[prost(bytes="vec", tag="6")]
    pub new_default_value: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditDataFieldForTagTypeResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TagDataType {
    ///选项
    Option = 0,
    /// 文字
    Text = 1,
    /// 时长
    Dueration = 2,
    ///日期
    Date = 3,
    /// 日期时间
    DateTime = 4,
    /// 量, 带有单位
    Quantity = 5,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tag {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(float, tag="2")]
    pub ammount: f32,
}
/// 添加标签
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddTagRequest {
    #[prost(string, tag="1")]
    pub tag_map_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub point_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub tag_type_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="4")]
    pub tag_value: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddTagResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 设置标签值
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetTagValueRequest {
    #[prost(string, tag="1")]
    pub tag_map_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub point_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub tag_type_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="4")]
    pub tag_value: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetTagValueResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 移除标签
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveTagRequest {
    #[prost(string, tag="1")]
    pub tag_map_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub point_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub tag_type_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveTagResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTagMapRequest {
    #[prost(string, tag="1")]
    pub roadmap_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub language: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTagMapResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewCommentRequest {
    #[prost(string, tag="1")]
    pub target_manage_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub target_entity_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub contents: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewCommentResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditCommentRequest {
    #[prost(string, tag="1")]
    pub comment_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub new_contents: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditCommentResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveCommentRequest {
    #[prost(string, tag="1")]
    pub target_manage_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub target_entity_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub comment_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveCommentResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 问题属于图
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewQuestionRequest {
    #[prost(string, tag="1")]
    pub graph_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub contents: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewQuestionResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditQuestionRequest {
    #[prost(string, tag="2")]
    pub question_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub new_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub new_contents: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditQuestionResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveQuestionRequest {
    #[prost(string, tag="1")]
    pub graph_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub question_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveQuestionResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAnswerRequest {
    #[prost(string, tag="1")]
    pub question_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub contents: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAnswerResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditAnswerRequest {
    #[prost(string, tag="1")]
    pub answer_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub new_contents: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditAnswerResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveAnswerRequest {
    #[prost(string, tag="1")]
    pub question_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub answer_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveAnswerResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Season {
    Spring = 0,
    Summer = 1,
    Autumn = 2,
    Winter = 3,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Position {
    #[prost(int32, tag="1")]
    pub x: i32,
    #[prost(int32, tag="2")]
    pub y: i32,
}
/// Generated server implementations.
pub mod cashmere_grpc_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with CashmereGrpcServer.
    #[async_trait]
    pub trait CashmereGrpc: Send + Sync + 'static {}
    #[derive(Debug)]
    pub struct CashmereGrpcServer<T: CashmereGrpc> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: CashmereGrpc> CashmereGrpcServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CashmereGrpcServer<T>
    where
        T: CashmereGrpc,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: CashmereGrpc> Clone for CashmereGrpcServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: CashmereGrpc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: CashmereGrpc> tonic::transport::NamedService for CashmereGrpcServer<T> {
        const NAME: &'static str = "cashmere.CashmereGrpc";
    }
}
