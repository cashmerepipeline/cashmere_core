/// 名
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Name {
    #[prost(string, tag = "1")]
    pub language: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
/// 名属性，语言：语言名
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NameField {
    #[prost(map = "string, string", tag = "1")]
    pub name_field: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// 重命名
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(string, tag = "2")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub new_name: ::core::option::Option<Name>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 新语言名
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewLanguageNameRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(string, tag = "2")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub new_name: ::core::option::Option<Name>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewLanguageNameResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 移除语言名
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLanguageNameRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(string, tag = "2")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub language: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLanguageNameResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 新品牌
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewCountryRequest {
    #[prost(message, optional, tag = "1")]
    pub name: ::core::option::Option<Name>,
    #[prost(string, tag = "2")]
    pub native: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub code: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub phone_area_code: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "5")]
    pub languages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewCountryResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LanguageCode {
    #[prost(string, tag = "1")]
    pub code: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub name_map: ::prost::alloc::vec::Vec<u8>,
}
/// 新语言编码
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewLanguageCodeRequest {
    #[prost(message, optional, tag = "1")]
    pub name: ::core::option::Option<Name>,
    #[prost(string, tag = "2")]
    pub code: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub native_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewLanguageCodeResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// TODO: 可能不需要
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditLanguageCodeRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub new_code: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub new_native: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditLanguageCodeResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewGroupRequest {
    #[prost(message, optional, tag = "1")]
    pub name: ::core::option::Option<Name>,
    #[prost(string, tag = "2")]
    pub new_group_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewGroupResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int32Range {
    #[prost(int32, tag = "1")]
    pub min: i32,
    #[prost(int32, tag = "2")]
    pub max: i32,
    #[prost(int32, tag = "3")]
    pub value: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int64Range {
    #[prost(int64, tag = "1")]
    pub min: i64,
    #[prost(int64, tag = "2")]
    pub max: i64,
    #[prost(int64, tag = "3")]
    pub value: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UInt32Range {
    #[prost(uint32, tag = "1")]
    pub min: u32,
    #[prost(uint32, tag = "2")]
    pub max: u32,
    #[prost(uint32, tag = "3")]
    pub value: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloatRange {
    #[prost(float, tag = "1")]
    pub min: f32,
    #[prost(float, tag = "2")]
    pub max: f32,
    #[prost(float, tag = "3")]
    pub value: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoubleRange {
    #[prost(double, tag = "1")]
    pub min: f64,
    #[prost(double, tag = "2")]
    pub max: f64,
    #[prost(double, tag = "3")]
    pub value: f64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Color {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub red: u32,
    #[prost(uint32, tag = "3")]
    pub green: u32,
    #[prost(uint32, tag = "4")]
    pub blue: u32,
    #[prost(uint32, tag = "5")]
    pub alpha: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Gender {
    Male = 0,
    Female = 1,
}
impl Gender {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Gender::Male => "Male",
            Gender::Female => "Female",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Male" => Some(Self::Male),
            "Female" => Some(Self::Female),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataServerConfigs {
    #[prost(string, tag = "1")]
    pub root_dir_path: ::prost::alloc::string::String,
    /// 文件最大大小, 16MB
    #[prost(uint64, tag = "2")]
    pub max_file_size: u64,
    /// 文件集最大数量, 1000
    #[prost(uint32, tag = "3")]
    pub max_set_size: u32,
    /// 文件序列最大数量
    #[prost(uint64, tag = "4")]
    pub max_sequence_length: u64,
    /// 最大文件上传连接
    #[prost(uint32, tag = "5")]
    pub max_file_upload_number: u32,
    /// 最大文件下载连接
    #[prost(uint32, tag = "6")]
    pub max_file_download_number: u32,
    /// 块最大大小，1024*128=128KB
    #[prost(uint32, tag = "7")]
    pub transfer_chunk_size: u32,
    /// 内部文件路径，不需要通过服务器上传文件, 可将文件直接存储到目标位置
    /// {"windows"="X:/data_root/dir", "linux"="/mnt/data_root/dir", "macos" = "/mnt/data_root/dir"}
    #[prost(map = "string, string", tag = "8")]
    pub internal_root_dir_map: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// 取得数据服务设置
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataServerConfigsRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataServerConfigsResponse {
    #[prost(message, optional, tag = "1")]
    pub configs: ::core::option::Option<DataServerConfigs>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataInfo {
    #[prost(enumeration = "DataType", tag = "1")]
    pub data_type: i32,
    #[prost(int32, tag = "3")]
    pub owner_manage_id: i32,
    #[prost(string, tag = "4")]
    pub owner_entity_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "5")]
    pub specs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// 新建数据
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewDataRequest {
    #[prost(message, optional, tag = "1")]
    pub name: ::core::option::Option<Name>,
    #[prost(enumeration = "DataType", tag = "2")]
    pub data_type: i32,
    #[prost(int32, tag = "3")]
    pub owner_manage_id: i32,
    #[prost(string, tag = "4")]
    pub owner_entity_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewDataResponse {
    /// 成功返回 "ok"
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 取得数据信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataInfoRequest {
    #[prost(string, tag = "1")]
    pub data_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub data_info: ::core::option::Option<DataInfo>,
}
/// 删除数据
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkDataRemovedRequest {
    #[prost(int32, tag = "1")]
    pub owner_manage_id: i32,
    #[prost(string, tag = "2")]
    pub owner_entity_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub data_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkDataRemovedResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 取得实体数据表
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntityDataRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(string, tag = "2")]
    pub entity_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntityDataResponse {
    #[prost(string, repeated, tag = "1")]
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
impl DataType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DataType::FileData => "FileData",
            DataType::SequenceData => "SequenceData",
            DataType::FileSetData => "FileSetData",
            DataType::DocumentData => "DocumentData",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FileData" => Some(Self::FileData),
            "SequenceData" => Some(Self::SequenceData),
            "FileSetData" => Some(Self::FileSetData),
            "DocumentData" => Some(Self::DocumentData),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSpecsRequest {
    #[prost(string, tag = "2")]
    pub data_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub name: ::core::option::Option<Name>,
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSpecsResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSpecsRequest {
    #[prost(string, tag = "1")]
    pub data_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSpecsResponse {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub specses: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSpecsPrefabsRequest {
    #[prost(string, tag = "1")]
    pub specs_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSpecsPrefabsResponse {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub prefabs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Version {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// protobuf不支持嵌套repeated，所以使用 “,” 分隔的字符串, 形式为["sub_dir, ...,file_name"]
    /// 路径不允许使用相对路径符号"."和".."
    /// 文件集为多个文件列表
    /// 文件序列为规则: ["base_name, start, end, padding, extension"]
    /// 使用bson格式存储
    #[prost(string, repeated, tag = "2")]
    pub files: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 移除标记，文件不删除
    #[prost(bool, tag = "3")]
    pub removed: bool,
}
/// 添加数据版本到阶段
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddStageVersionRequest {
    #[prost(string, tag = "1")]
    pub stage_id: ::prost::alloc::string::String,
    /// 版本一般有具体的含义，不只是一个数字，比如"v001", 数据的名应该与版本一致
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddStageVersionResponse {
    /// 成功返回 "ok"
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 取得数据阶段版本表
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStageVersionsRequest {
    #[prost(string, tag = "1")]
    pub stage_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStageVersionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub versions: ::prost::alloc::vec::Vec<Version>,
}
/// 改变阶段文件连接
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetStageCurrentVersionRequest {
    #[prost(string, tag = "1")]
    pub stage_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub target_version: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetStageCurrentVersionResponse {
    /// 成功返回 当前版本
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 删除数据版本
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveStageVersionRequest {
    #[prost(string, tag = "1")]
    pub stage_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveStageVersionResponse {
    /// 成功返回 "ok"
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 添加文件到数据阶段，文件路径以版本路径为根，<version_root>/["sub_dir", ..., "file_name"]
/// 路径在使用时再拼接
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddFileToVersionRequest {
    #[prost(string, tag = "1")]
    pub stage_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub file_path: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddFileToVersionResponse {
    /// 成功返回 "ok"
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 添加文件到数据阶段，文件路径以版本路径为根，<version_root>/["sub_dir", ..., "file_name"]
/// 路径在使用时再拼接
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddFileSetToVersionRequest {
    #[prost(string, tag = "1")]
    pub stage_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    /// 因为不支持嵌套repeated，所以使用“,”分隔的字符串, 形式为["sub_dir, ...,file_name"]
    #[prost(string, repeated, tag = "3")]
    pub file_pathes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddFileSetToVersionResponse {
    /// 成功返回 "ok"
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 数据类型为文件序列时, 序列文件直接存储在版本目录下
/// 使用规则解析文件路径[base_name, start, end, padding, extension]，不记录所有文件的路径
/// 严格使用这个顺序，不使用类似{base_name: name, start: start, end: end, padding: padding, ext: ext, number_pos: [mid, end]}的格式
/// 文件、文件集、序列存储形式上一致，易于mongodb文件查询
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddFileSequenceToVersionRequest {
    #[prost(string, tag = "1")]
    pub stage_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub base_name: ::prost::alloc::string::String,
    #[prost(int32, tag = "5")]
    pub start: i32,
    #[prost(int32, tag = "6")]
    pub end: i32,
    #[prost(int32, tag = "2")]
    pub padding: i32,
    #[prost(string, tag = "8")]
    pub extension: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddFileSequenceToVersionResponse {
    /// 成功返回 "ok"
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 从版本中移除多个文件，只支持文件、文件集， 不支持文件序列
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveFilesFromVersionRequest {
    #[prost(string, tag = "1")]
    pub stage_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub file_pathes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveFilesFromVersionResponse {
    /// 成功返回被删除的文件路径
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 列出版本文件目录下的文件和文件夹
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVersionFolderRequest {
    #[prost(string, tag = "1")]
    pub stage_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVersionFolderResponse {
    /// 文件夹
    #[prost(string, repeated, tag = "1")]
    pub folders: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// 文件
    #[prost(string, repeated, tag = "2")]
    pub files: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// 删除版本目录下的文件或文件夹，若文件或文件夹在版本文件列表中，否则返回，不做任何操作
/// 注意路径表示规则
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteVersionFolderEntriesRequest {
    #[prost(string, tag = "1")]
    pub stage_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub file_pathes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteVersionFolderEntriesResponse {
    /// 成功返回被删除的文件路径
    #[prost(string, repeated, tag = "1")]
    pub result: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// 数据阶段信息
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StageInfo {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// stage目录下的文件列表, 文件为文件列表，集合为集合目录列表
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub versions: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// 当前连接所指版本
    #[prost(string, tag = "3")]
    pub current_version: ::prost::alloc::string::String,
}
/// 新阶段
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewStageRequest {
    #[prost(string, tag = "1")]
    pub specs_id: ::prost::alloc::string::String,
    /// 这里可能因为软件对路径字符集支持的不同只能使用英文作为文件名，比如Maya
    #[prost(message, optional, tag = "2")]
    pub stage_name: ::core::option::Option<Name>,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewStageResponse {
    /// 成功返回 "ok"
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 删除阶段，只删除阶段->数据的文件连接，不删除数据
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveStageRequest {
    #[prost(string, tag = "1")]
    pub specs_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub stage: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveStageResponse {
    /// 成功返回 "ok"
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 取得数据阶段表
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStagesRequest {
    #[prost(string, tag = "1")]
    pub specs_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStagesResponse {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub stages: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewPrefabRequest {
    #[prost(string, tag = "1")]
    pub specs_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub stage_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub name: ::core::option::Option<Name>,
    /// 使用bson格式存储修改信息
    #[prost(bytes = "vec", tag = "5")]
    pub modifies: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewPrefabResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 列出预制件
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPrefabRequest {
    #[prost(string, tag = "1")]
    pub specs_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPrefabResponse {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub prefabs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 文件信息
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileInfo {
    #[prost(string, tag = "1")]
    pub file_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub md5: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub size: u64,
    #[prost(int64, tag = "4")]
    pub last_modified_time: i64,
}
/// 上传文件数据
/// 第一个包块编号为0，最后一个包块编号为0, 即从0开始，到0结束
/// 第一个包和最后一个包不包含文件数据，作为传输标记用
/// 最终路径为：/{data_id}/{specs}/{stage}/{version}/{sub_path}/{file_name}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadFileRequest {
    #[prost(string, tag = "1")]
    pub data_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub total_chunks: u64,
    #[prost(uint64, tag = "3")]
    pub current_chunk_index: u64,
    #[prost(bytes = "vec", tag = "4")]
    pub chunk: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "6")]
    pub chunk_md5: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub file_info: ::core::option::Option<FileInfo>,
    /// 规格，如：普通款，高级款，豪华款
    #[prost(string, tag = "10")]
    pub specs: ::prost::alloc::string::String,
    /// 阶段，如：开发，测试，生产
    #[prost(string, tag = "8")]
    pub stage: ::prost::alloc::string::String,
    /// 版本，如：v01
    #[prost(string, tag = "9")]
    pub version: ::prost::alloc::string::String,
    /// 相对于版本目录的子路径，用于保持文件集的相对良好组织，如：v01/a/b/c
    #[prost(string, tag = "11")]
    pub sub_path: ::prost::alloc::string::String,
}
/// 下一个包块编号
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadFileResponse {
    #[prost(uint64, tag = "1")]
    pub next_chunk_index: u64,
}
/// 下载文件数据
/// 编号为0请求返回文件信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadFileRequest {
    #[prost(string, tag = "1")]
    pub data_id: ::prost::alloc::string::String,
    /// 相对数据存储根目录
    #[prost(string, tag = "6")]
    pub specs: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub stage: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub chunk_index: u64,
    /// 如果给出版本，则下载对应版本的文件，没有则下载阶段软连接指向的文件
    #[prost(string, tag = "4")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub sub_path: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub file_name: ::prost::alloc::string::String,
}
/// 返回文件流
/// 最后一个包编号为0, 表示下载完成
/// 最后一个包不包含文件数据，作为传输标记用
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadFileResponse {
    #[prost(string, tag = "1")]
    pub data_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub chunk_index: u64,
    #[prost(bytes = "vec", tag = "4")]
    pub chunk: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "5")]
    pub chunk_md5: ::prost::alloc::string::String,
}
/// 序列数据信息
/// 文件名格式：prefix_name.pattern.type_suffix
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SequenceDataInfo {
    #[prost(string, tag = "1")]
    pub prefix_name: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub sequence_pattern: u64,
    #[prost(uint64, tag = "3")]
    pub type_suffix: u64,
    #[prost(uint64, tag = "4")]
    pub start_index: u64,
    #[prost(uint64, tag = "5")]
    pub end_index: u64,
    #[prost(uint64, tag = "6")]
    pub total_size: u64,
    #[prost(string, tag = "7")]
    pub md5: ::prost::alloc::string::String,
}
/// 上传文件序列
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SequenceDataUploadSequenceRequest {
    #[prost(string, tag = "1")]
    pub sequence_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub serial_pattern: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub sequence_length: u32,
    #[prost(string, tag = "4")]
    pub current_file: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub total_chancks: u64,
    #[prost(uint64, tag = "6")]
    pub current_chunck: u64,
    #[prost(bytes = "vec", tag = "7")]
    pub chunck: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "8")]
    pub data_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SequenceDataUploadSequenceResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 下载文件序列
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SequenceDataDownloadSequenceRequest {
    #[prost(string, tag = "1")]
    pub data_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SequenceDataDownloadSequenceResponse {
    #[prost(string, tag = "8")]
    pub data_id: ::prost::alloc::string::String,
    #[prost(string, tag = "1")]
    pub sequence_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub serial_pattern: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub sequence_length: u32,
    #[prost(string, tag = "4")]
    pub current_file: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub total_chancks: u64,
    #[prost(uint64, tag = "6")]
    pub current_chunck: u64,
    #[prost(bytes = "vec", tag = "7")]
    pub chunck: ::prost::alloc::vec::Vec<u8>,
}
/// 集合数据信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDataInfo {
    #[prost(string, tag = "1")]
    pub data_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub total_size: u64,
    #[prost(message, repeated, tag = "3")]
    pub file_infos: ::prost::alloc::vec::Vec<FileInfo>,
}
/// 取得数据集合信息
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSetDataInfoRequest {
    #[prost(string, tag = "1")]
    pub data_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSetDataInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub data_info: ::core::option::Option<SetDataInfo>,
}
/// 上传单个文件到集合
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDataUploadFileRequest {
    #[prost(string, tag = "1")]
    pub data_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub total_chunks: u64,
    #[prost(uint64, tag = "3")]
    pub current_chunk_index: u64,
    #[prost(bytes = "vec", tag = "4")]
    pub chunk: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "5")]
    pub file_info: ::core::option::Option<FileInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDataUploadFileResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 上传多个文件到集合, 批量上传可能不是很需要
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDataUploadFilesRequest {
    #[prost(string, tag = "1")]
    pub data_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub current_total_chunks: u64,
    #[prost(uint64, tag = "3")]
    pub current_chunk_index: u64,
    #[prost(bytes = "vec", tag = "4")]
    pub chunk: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "5")]
    pub file_info: ::prost::alloc::vec::Vec<FileInfo>,
    #[prost(string, tag = "6")]
    pub current_md5: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDataUploadFilesResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 下载文件集合
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDataDownloadSetRequest {
    #[prost(string, tag = "1")]
    pub data_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDataDownloadSetResponse {
    #[prost(string, tag = "1")]
    pub data_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub set_name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub file_counts: u32,
    #[prost(string, tag = "4")]
    pub current_file: ::prost::alloc::string::String,
    #[prost(uint64, tag = "5")]
    pub total_chancks: u64,
    #[prost(uint32, tag = "6")]
    pub current_chunck: u32,
    #[prost(bytes = "vec", tag = "7")]
    pub chunck: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAreaRequest {
    #[prost(message, optional, tag = "1")]
    pub name: ::core::option::Option<Name>,
    #[prost(string, tag = "2")]
    pub parent_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub code: ::prost::alloc::string::String,
    #[prost(enumeration = "AreaLevel", tag = "4")]
    pub level: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAreaResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditAreaRequest {
    #[prost(string, tag = "1")]
    pub area_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub new_parent_id: ::prost::alloc::string::String,
    #[prost(enumeration = "AreaLevel", tag = "4")]
    pub new_level: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditAreaResponse {
    #[prost(string, tag = "1")]
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
impl AreaLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AreaLevel::Country => "Country",
            AreaLevel::Province => "Province",
            AreaLevel::City => "City",
            AreaLevel::Area => "Area",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Country" => Some(Self::Country),
            "Province" => Some(Self::Province),
            "City" => Some(Self::City),
            "Area" => Some(Self::Area),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entity {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub name: ::core::option::Option<Name>,
    #[prost(string, tag = "3")]
    pub creator_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub create_timestamp: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub modifier_id: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub modify_timestamp: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub owner_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "8")]
    pub groups: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "9")]
    pub data_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "10")]
    pub comment_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag = "11")]
    pub removed: bool,
    #[prost(string, repeated, tag = "12")]
    pub removed_data_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// 变更物主
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeOwnerRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(string, tag = "2")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub old_owner_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub new_owner_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeOwnerResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 太通用，不建议开放
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEntityRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEntityResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 不建议开放
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(string, tag = "2")]
    pub entity_id: ::prost::alloc::string::String,
    /// {field_id:value, ...}
    #[prost(bytes = "vec", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 通用修改实体属性
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityFieldRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(string, tag = "2")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub field_id: ::prost::alloc::string::String,
    /// {field_id:value}
    #[prost(bytes = "vec", tag = "4")]
    pub new_value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityFieldResponse {
    /// 成功返回新值
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 通用修改MAP实体属性
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityMapFieldRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(string, tag = "2")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub field_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub key: ::prost::alloc::string::String,
    /// {key:value}
    #[prost(bytes = "vec", tag = "5")]
    pub new_value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityMapFieldResponse {
    /// 成功返回新值
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 通用修改MAP移除key
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityMapFieldRemoveKeyRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(string, tag = "2")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub field_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityMapFieldRemoveKeyResponse {
    /// 成功返回key
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 通用修改List实体属性, 添加成员
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityArrayFieldAddItemsRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(string, tag = "2")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub field_id: ::prost::alloc::string::String,
    /// {field_id:\[items\]}
    #[prost(bytes = "vec", tag = "4")]
    pub items: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityArrayFieldAddItemsResponse {
    /// 成功返回"ok"
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 通用修改List实体属性, 移除物体
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityArrayFieldRemoveItemsRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(string, tag = "2")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub field_id: ::prost::alloc::string::String,
    /// {field_id:\[items\]}
    #[prost(bytes = "vec", tag = "4")]
    pub items: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityArrayFieldRemoveItemsResponse {
    /// 成功返回"ok"
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 取得实体
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEntityRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(string, tag = "2")]
    pub entity_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEntityResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub entity: ::prost::alloc::vec::Vec<u8>,
}
/// 取得多个实体
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEntitiesRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(string, repeated, tag = "2")]
    pub entity_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEntitiesResponse {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub entities: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 取得实体页
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEntitiesPageRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(uint32, tag = "2")]
    pub page_index: u32,
    #[prost(bytes = "vec", tag = "3")]
    pub conditions: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEntitiesPageResponse {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub entities: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 标记实体已移除
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkEntityRemovedRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(string, tag = "2")]
    pub entity_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkEntityRemovedResponse {
    /// 成功返回"ok"
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 恢复标记为已移除的实体
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecoverRemovedEntityRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(string, tag = "2")]
    pub entity_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecoverRemovedEntityResponse {
    /// 成功返回"ok"
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 取得已删除实体页
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRemovedEntitiesPageRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(uint32, tag = "2")]
    pub page_index: u32,
    #[prost(bytes = "vec", tag = "3")]
    pub conditions: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRemovedEntitiesPageResponse {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub entities: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 取得实体已标记移除数据表
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRemovedDataListRequest {
    #[prost(string, tag = "1")]
    pub manage_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub data_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRemovedDataListResponse {
    #[prost(string, repeated, tag = "1")]
    pub data_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEntityTemplateRequest {
    /// 模板对应管理
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    /// 属性:值 列表
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub fields: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEntityTemplateResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityTemplateRequest {
    /// 模板编号
    #[prost(string, tag = "1")]
    pub template_id: ::prost::alloc::string::String,
    /// 属性:值 列表
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub fields: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityTemplateResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveEntityTemplateRequest {
    #[prost(string, tag = "1")]
    pub template_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveEntityTemplateResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Manage {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(bytes = "vec", tag = "2")]
    pub name_map: ::prost::alloc::vec::Vec<u8>,
}
/// 取得管理列表
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManagesRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManagesResponse {
    #[prost(message, repeated, tag = "1")]
    pub manages: ::prost::alloc::vec::Vec<Manage>,
}
/// 取得记录数量
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManageEntryCountRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManageEntryCountResponse {
    #[prost(uint64, tag = "1")]
    pub count: u64,
}
/// 映像请求
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManageViewRequest {
    #[prost(string, tag = "1")]
    pub manage_name: ::prost::alloc::string::String,
}
/// 映像返回
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManageViewResponse {
    #[prost(string, tag = "1")]
    pub view_token: ::prost::alloc::string::String,
}
/// 管理权限
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeManageReadRuleRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(string, tag = "2")]
    pub group_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub read_rule: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeManageReadRuleResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeManageWriteRuleRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(string, tag = "2")]
    pub group_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub write_rule: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeManageWriteRuleResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 集合权限
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeCollectionReadRuleRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(string, tag = "2")]
    pub group_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub read_rule: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeCollectionReadRuleResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeCollectionWriteRuleRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(string, tag = "2")]
    pub group_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub write_rule: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeCollectionWriteRuleResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 描写字段权限
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeFieldReadRuleRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(string, tag = "2")]
    pub group_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub field_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub read_rule: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeFieldReadRuleResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeFieldWriteRuleRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(string, tag = "2")]
    pub group_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub field_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub write_rule: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeFieldWriteRuleResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SchemaField {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(bytes = "vec", tag = "2")]
    pub name_map: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub data_type: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub removed: bool,
}
/// 取得管理描写
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManageSchemaRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManageSchemaResponse {
    #[prost(message, repeated, tag = "1")]
    pub fields: ::prost::alloc::vec::Vec<SchemaField>,
}
/// 添加管理属性
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSchemaFieldRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(message, optional, tag = "2")]
    pub field: ::core::option::Option<SchemaField>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSchemaFieldResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 标记属性移除
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkSchemaFieldRemovedRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(int32, tag = "2")]
    pub field_id: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkSchemaFieldRemovedResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 编辑属性名
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditSchemaFieldNameRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(int32, tag = "2")]
    pub field_id: i32,
    #[prost(string, tag = "3")]
    pub language: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub new_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditSchemaFieldNameResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewTagTypeRequest {
    #[prost(string, tag = "1")]
    pub domain_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub language: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewTagTypeResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddDataFieldToTagTypeRequest {
    #[prost(string, tag = "1")]
    pub tag_type_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub language: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub field_name: ::prost::alloc::string::String,
    #[prost(enumeration = "TagDataType", tag = "4")]
    pub data_type: i32,
    #[prost(bytes = "vec", tag = "5")]
    pub default_value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddDataFieldToTagTypeResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveDataFieldFromTagTypeRequest {
    #[prost(string, tag = "1")]
    pub tag_type_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub language: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub field_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveDataFieldFromTagTypeResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditDataFieldForTagTypeRequest {
    #[prost(string, tag = "1")]
    pub tag_type_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub language: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub old_field_name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub new_field_name: ::prost::alloc::string::String,
    #[prost(enumeration = "TagDataType", tag = "5")]
    pub new_data_type: i32,
    #[prost(bytes = "vec", tag = "6")]
    pub new_default_value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditDataFieldForTagTypeResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TagDataType {
    /// 选项
    Option = 0,
    /// 文字
    Text = 1,
    /// 时长
    Dueration = 2,
    /// 日期
    Date = 3,
    /// 日期时间
    DateTime = 4,
    /// 量, 带有单位
    Quantity = 5,
}
impl TagDataType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TagDataType::Option => "Option",
            TagDataType::Text => "Text",
            TagDataType::Dueration => "Dueration",
            TagDataType::Date => "Date",
            TagDataType::DateTime => "DateTime",
            TagDataType::Quantity => "Quantity",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Option" => Some(Self::Option),
            "Text" => Some(Self::Text),
            "Dueration" => Some(Self::Dueration),
            "Date" => Some(Self::Date),
            "DateTime" => Some(Self::DateTime),
            "Quantity" => Some(Self::Quantity),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tag {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(float, tag = "2")]
    pub ammount: f32,
}
/// 添加标签
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddTagRequest {
    #[prost(string, tag = "1")]
    pub tag_map_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub point_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub tag_type_id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub tag_value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddTagResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 设置标签值
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetTagValueRequest {
    #[prost(string, tag = "1")]
    pub tag_map_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub point_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub tag_type_id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub tag_value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetTagValueResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 移除标签
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveTagRequest {
    #[prost(string, tag = "1")]
    pub tag_map_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub point_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub tag_type_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveTagResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTagMapRequest {
    #[prost(string, tag = "1")]
    pub roadmap_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub language: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTagMapResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewCommentRequest {
    #[prost(string, tag = "1")]
    pub target_manage_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub target_entity_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub contents: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewCommentResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditCommentRequest {
    #[prost(string, tag = "1")]
    pub comment_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub new_contents: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditCommentResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveCommentRequest {
    #[prost(string, tag = "1")]
    pub target_manage_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub target_entity_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub comment_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveCommentResponse {
    #[prost(string, tag = "1")]
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
impl Season {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Season::Spring => "Spring",
            Season::Summer => "Summer",
            Season::Autumn => "Autumn",
            Season::Winter => "Winter",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Spring" => Some(Self::Spring),
            "Summer" => Some(Self::Summer),
            "Autumn" => Some(Self::Autumn),
            "Winter" => Some(Self::Winter),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Position {
    #[prost(int32, tag = "1")]
    pub x: i32,
    #[prost(int32, tag = "2")]
    pub y: i32,
}
