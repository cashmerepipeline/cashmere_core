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
    #[prost(string, tag="1")]
    pub manage_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub language: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub new_name: ::prost::alloc::string::String,
}
///*
///@returns 成功返回新名字, 其他情况返回其他
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 新语言名
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewLanguageNameRequest {
    #[prost(string, tag="1")]
    pub manage_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub language: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub new_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewLanguageNameResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
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
/// 新建数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewDataRequest {
    #[prost(message, optional, tag="1")]
    pub name: ::core::option::Option<Name>,
    #[prost(enumeration="DataType", tag="2")]
    pub data_type: i32,
    #[prost(int32, tag="3")]
    pub manage_id: i32,
    #[prost(string, tag="4")]
    pub entity_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewDataResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 上传文件数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileDataUploadFileRequest {
    #[prost(message, optional, tag="6")]
    pub name: ::core::option::Option<Name>,
    #[prost(string, tag="1")]
    pub data_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub total_chuncks: u64,
    #[prost(uint64, tag="3")]
    pub current_chunck: u64,
    #[prost(bytes="vec", tag="4")]
    pub chunck: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="5")]
    pub file_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileDataUploadFileResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 下载文件数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileDataDownloadFileRequest {
    #[prost(string, tag="1")]
    pub data_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileDataDownloadFileResponse {
    #[prost(string, tag="1")]
    pub data_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub total_chuncks: u64,
    #[prost(uint64, tag="3")]
    pub current_chunck: u64,
    #[prost(bytes="vec", tag="4")]
    pub chunck: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="5")]
    pub file_name: ::prost::alloc::string::String,
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
/// 上传文件集合
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDataUploadSetRequest {
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
pub struct SetDataUploadSetResponse {
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
/// 关联数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssociateDataRequest {
    #[prost(string, tag="1")]
    pub data_id: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub manage_id: i32,
    #[prost(string, tag="3")]
    pub entity_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssociateDataResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 取消关联数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisassociateDataRequest {
    #[prost(string, tag="1")]
    pub data_id: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub manage_id: i32,
    #[prost(string, tag="3")]
    pub entity_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisassociateDataResponse {
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
    #[prost(bytes="vec", tag="1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// 删除数据
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkDataRemovedRequest {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    #[prost(string, tag="2")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub data_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkDataRemovedResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DataType {
    /// 单个文件
    File = 0,
    /// 序列文件
    Sequence = 1,
    /// 文件集合
    Set = 2,
    /// 消息
    Message = 3,
    /// 数据库数据
    Document = 4,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAreaRequest {
    #[prost(string, tag="1")]
    pub language: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub parent_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub code: ::prost::alloc::string::String,
    #[prost(enumeration="AreaLevel", tag="5")]
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
    /// bson document {id, name, writeable_fields, readonly_fields}
    #[prost(bytes="vec", tag="1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityRequest {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    #[prost(string, tag="2")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 标记实体已移除
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkEntityRemovedRequest {
    #[prost(string, tag="1")]
    pub manage_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub entity_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkEntityRemovedResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 取得实体数据表
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataListRequest {
    #[prost(int32, tag="1")]
    pub manage_id: i32,
    #[prost(string, tag="2")]
    pub entity_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataListResponse {
    #[prost(string, repeated, tag="1")]
    pub data_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Field {
    #[prost(int32, tag="1")]
    pub id: i32,
    #[prost(bytes="vec", tag="2")]
    pub name: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub data_type: ::prost::alloc::string::String,
    #[prost(bool, tag="4")]
    pub removed: bool,
}
/// 取得管理列表
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManagesRequest {
}
/// 取得管理描写
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManageSchemaRequest {
    #[prost(string, tag="1")]
    pub manage_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManageSchemaResponse {
    #[prost(bytes="vec", repeated, tag="1")]
    pub schema: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 添加管理属性
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSchemaFieldRequest {
    #[prost(string, tag="1")]
    pub manage_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub field: ::core::option::Option<Field>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSchemaFieldResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 移除管理属性
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveSchemaFieldRequest {
    #[prost(string, tag="1")]
    pub manage_id: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub field_id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveSchemaFieldResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 编辑管理属性
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditSchemaFieldNameRequest {
    #[prost(string, tag="1")]
    pub manage_id: ::prost::alloc::string::String,
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
/// 取得记录数量
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManageEntryCountRequest {
    #[prost(string, tag="1")]
    pub manage_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManageEntryCountResponse {
    #[prost(uint64, tag="1")]
    pub count: u64,
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
/// 添加
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewPhaseForWorkRequest {
    #[prost(string, tag="1")]
    pub work_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub phase_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewPhaseForWorkResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewWorkNodeForProcedureGraphRequest {
    #[prost(string, tag="1")]
    pub procedure_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub node_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewWorkNodeForProcedureGraphResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkNodeLinkRequest {
    #[prost(string, tag="1")]
    pub procedure_id: ::prost::alloc::string::String,
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
    pub procedure_id: ::prost::alloc::string::String,
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssignWorkNodeToWorkerRequest {
    #[prost(string, tag="1")]
    pub work_node_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub worker_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssignWorkNodeToWorkerResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewTaskForWorkNodeRequest {
    #[prost(string, tag="1")]
    pub work_node_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewTaskForWorkNodeResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewDataForTaskRequest {
    #[prost(string, tag="1")]
    pub task_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewDataForTaskResponse {
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewWorkPhaseRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewWorkPhaseResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditWorkPhaseRequest {
    #[prost(string, tag="1")]
    pub phase_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub new_phase: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditWorkPhaseResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
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