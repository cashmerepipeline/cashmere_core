#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameRequest {
    #[prost(string, tag = "1")]
    pub manage_id: std::string::String,
    #[prost(string, tag = "2")]
    pub entity_id: std::string::String,
    #[prost(string, tag = "3")]
    pub local: std::string::String,
    #[prost(string, tag = "4")]
    pub new_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewLocalNameRequest {
    #[prost(string, tag = "1")]
    pub manage_id: std::string::String,
    #[prost(string, tag = "2")]
    pub entity_id: std::string::String,
    #[prost(string, tag = "3")]
    pub local: std::string::String,
    #[prost(string, tag = "4")]
    pub new_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewLocalNameResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entity {
    /// bson document {id, name, writeable_fields, readonly_fields}
    #[prost(bytes, tag = "1")]
    pub data: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEntityRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(bytes, tag = "2")]
    pub data: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEntityResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(string, tag = "2")]
    pub entity_id: std::string::String,
    #[prost(bytes, tag = "3")]
    pub data: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveEntityRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(string, tag = "2")]
    pub entity_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveEntityResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEntityTemplateRequest {
    /// 模板对应管理
    #[prost(string, tag = "1")]
    pub manage_id: std::string::String,
    /// 属性:值 列表
    #[prost(bytes, repeated, tag = "2")]
    pub fields: ::std::vec::Vec<std::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEntityTemplateResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityTemplateRequest {
    /// 模板编号
    #[prost(string, tag = "1")]
    pub template_id: std::string::String,
    /// 属性:值 列表
    #[prost(bytes, repeated, tag = "2")]
    pub fields: ::std::vec::Vec<std::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEntityTemplateResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveEntityTemplateRequest {
    #[prost(string, tag = "1")]
    pub template_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveEntityTemplateResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 映像请求
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManageViewRequest {
    #[prost(string, tag = "1")]
    pub manage_name: std::string::String,
}
/// 映像返回
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManageViewResponse {
    #[prost(string, tag = "1")]
    pub view_token: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Field {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(bytes, tag = "2")]
    pub name: std::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub data_type: std::string::String,
    #[prost(bool, tag = "4")]
    pub removed: bool,
}
/// 取得管理列表
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManagesRequest {}
/// 取得管理描写
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManageSchemaRequest {
    #[prost(string, tag = "1")]
    pub manage_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManageSchemaResponse {
    #[prost(bytes, repeated, tag = "1")]
    pub schema: ::std::vec::Vec<std::vec::Vec<u8>>,
}
/// 添加管理属性
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSchemaFieldRequest {
    #[prost(string, tag = "1")]
    pub manage_id: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub field: ::std::option::Option<Field>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSchemaFieldResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 移除管理属性
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveSchemaFieldRequest {
    #[prost(string, tag = "1")]
    pub manage_id: std::string::String,
    #[prost(int32, tag = "2")]
    pub field_id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveSchemaFieldResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 编辑管理属性
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditSchemaFieldNameRequest {
    #[prost(string, tag = "1")]
    pub manage_id: std::string::String,
    #[prost(int32, tag = "2")]
    pub field_id: i32,
    #[prost(string, tag = "3")]
    pub local: std::string::String,
    #[prost(string, tag = "4")]
    pub new_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditSchemaFieldNameResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(int32, tag = "1")]
    pub event_id: i32,
    #[prost(bytes, tag = "2")]
    pub context: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventQueue {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(string, tag = "2")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventHandle {
    #[prost(int32, tag = "1")]
    pub event_id: i32,
    #[prost(int32, tag = "2")]
    pub queue_id: i32,
    #[prost(int32, tag = "3")]
    pub manage_id: i32,
    #[prost(string, tag = "4")]
    pub name: std::string::String,
}
/// 新建事件
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEventRequest {
    #[prost(int32, tag = "2")]
    pub manage_id: i32,
    #[prost(string, tag = "1")]
    pub event_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEventResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 编辑 事件表
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEventRequest {
    #[prost(int32, tag = "1")]
    pub event_id: i32,
    #[prost(bytes, tag = "2")]
    pub new_values: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditEventResponse {
    #[prost(bool, tag = "1")]
    pub failed: bool,
}
/// 添加目标队列
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddEventTargetQueueRequest {
    #[prost(int32, tag = "1")]
    pub event_id: i32,
    #[prost(message, optional, tag = "3")]
    pub target: ::std::option::Option<EventQueue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddEventTargetQueueResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 新建事件队列
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEventQueueRequest {
    #[prost(int32, tag = "2")]
    pub manage_id: i32,
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEventQueueResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 新建事件处理
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEventHandleRequest {
    #[prost(int32, tag = "2")]
    pub event_id: i32,
    #[prost(int32, tag = "3")]
    pub queue_id: i32,
    #[prost(string, tag = "4")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEventHandleResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 触发事件
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmitEventRequest {
    #[prost(message, optional, tag = "1")]
    pub event: ::std::option::Option<Event>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmitEventResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 连接事件队列
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkEventQueueRequest {
    #[prost(int32, tag = "1")]
    pub queue_id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewWorkRequest {
    #[prost(string, tag = "1")]
    pub manage_id: std::string::String,
    #[prost(string, tag = "2")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewWorkResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
/// 添加
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewPhaseForWorkRequest {
    #[prost(string, tag = "1")]
    pub work_id: std::string::String,
    #[prost(string, tag = "2")]
    pub phase_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewPhaseForWorkResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewWorkNodeForProcedureGraphRequest {
    #[prost(string, tag = "1")]
    pub procedure_id: std::string::String,
    #[prost(string, tag = "2")]
    pub node_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewWorkNodeForProcedureGraphResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkNodeLinkRequest {
    #[prost(string, tag = "1")]
    pub procedure_id: std::string::String,
    #[prost(string, tag = "2")]
    pub start_node_id: std::string::String,
    #[prost(string, tag = "3")]
    pub out_slot: std::string::String,
    #[prost(string, tag = "4")]
    pub end_node_id: std::string::String,
    #[prost(string, tag = "5")]
    pub in_slot: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkNodeLinkResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveWorkNodeLinkRequest {
    #[prost(string, tag = "1")]
    pub procedure_id: std::string::String,
    #[prost(string, tag = "2")]
    pub start_node_id: std::string::String,
    #[prost(string, tag = "3")]
    pub out_slot: std::string::String,
    #[prost(string, tag = "4")]
    pub end_node_id: std::string::String,
    #[prost(string, tag = "5")]
    pub in_slot: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveWorkNodeLinkResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssignWorkNodeToWorkerRequest {
    #[prost(string, tag = "1")]
    pub work_node_id: std::string::String,
    #[prost(string, tag = "2")]
    pub worker_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssignWorkNodeToWorkerResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewTaskForWorkNodeRequest {
    #[prost(string, tag = "1")]
    pub work_node_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewTaskForWorkNodeResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewDataForTaskRequest {
    #[prost(string, tag = "1")]
    pub task_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewDataForTaskResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssociateDataToTaskRequest {
    #[prost(string, tag = "1")]
    pub data_id: std::string::String,
    #[prost(string, tag = "2")]
    pub task_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssociateDataToTaskResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitTaskRequest {
    #[prost(string, tag = "1")]
    pub task_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitTaskResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkTaskStatusRequest {
    #[prost(string, tag = "1")]
    pub task_id: std::string::String,
    #[prost(string, tag = "2")]
    pub status_set_id: std::string::String,
    #[prost(int32, tag = "3")]
    pub status_index: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkTaskStatusResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewWorkPhaseRequest {
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewWorkPhaseResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditWorkPhaseRequest {
    #[prost(string, tag = "1")]
    pub phase_id: std::string::String,
    #[prost(bytes, tag = "2")]
    pub new_phase: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditWorkPhaseResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewDataSlotForWorkNodeRequest {
    #[prost(string, tag = "1")]
    pub node_id: std::string::String,
    #[prost(enumeration = "SlotType", tag = "2")]
    pub slot_type: i32,
    #[prost(string, tag = "3")]
    pub slot_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewDataSlotForWorkNodeResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveDataSlotForWorkNodeRequest {
    #[prost(string, tag = "1")]
    pub node_id: std::string::String,
    #[prost(string, tag = "2")]
    pub slot_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveDataSlotForWorkNodeResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddDataToDataSlotRequest {
    #[prost(string, tag = "1")]
    pub node_id: std::string::String,
    #[prost(string, tag = "2")]
    pub slot_name: std::string::String,
    #[prost(string, tag = "3")]
    pub data_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddDataToDataSlotResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
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
pub struct NewPointRequest {
    #[prost(string, tag = "1")]
    pub domain_id: std::string::String,
    #[prost(string, tag = "2")]
    pub local: std::string::String,
    #[prost(string, tag = "3")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewPointResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenamePointRequest {
    #[prost(string, tag = "1")]
    pub point_id: std::string::String,
    #[prost(string, tag = "2")]
    pub local: std::string::String,
    #[prost(string, tag = "3")]
    pub new_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenamePointResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppendPointNameRequest {
    #[prost(string, tag = "1")]
    pub point_id: std::string::String,
    #[prost(string, tag = "2")]
    pub local: std::string::String,
    #[prost(string, tag = "3")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppendPointNameResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewGraphRequest {
    #[prost(string, tag = "1")]
    pub domain_id: std::string::String,
    #[prost(enumeration = "IndividualLevel", tag = "2")]
    pub level: i32,
    #[prost(string, tag = "3")]
    pub individual_id: std::string::String,
    #[prost(string, tag = "4")]
    pub local: std::string::String,
    #[prost(string, tag = "5")]
    pub name: std::string::String,
    #[prost(enumeration = "ComposeType", tag = "6")]
    pub compose_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewGraphResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameGraphRequest {
    #[prost(string, tag = "1")]
    pub graph_id: std::string::String,
    #[prost(string, tag = "2")]
    pub local: std::string::String,
    #[prost(string, tag = "3")]
    pub new_name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameGraphResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddEdgeToGraphRequest {
    #[prost(string, tag = "1")]
    pub graph_id: std::string::String,
    #[prost(string, tag = "2")]
    pub start_id: std::string::String,
    #[prost(string, tag = "3")]
    pub end_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddEdgeToGraphResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveEdgeFromGraphRequest {
    #[prost(string, tag = "1")]
    pub graph_id: std::string::String,
    #[prost(string, tag = "2")]
    pub start_id: std::string::String,
    #[prost(string, tag = "3")]
    pub end_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveEdgeFromGraphResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddTagToEdgeRequest {
    #[prost(string, tag = "1")]
    pub graph_id: std::string::String,
    #[prost(string, tag = "2")]
    pub start_id: std::string::String,
    #[prost(string, tag = "3")]
    pub end_id: std::string::String,
    #[prost(string, tag = "4")]
    pub tag_type: std::string::String,
    #[prost(double, tag = "5")]
    pub value: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddTagToEdgeResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IndividualLevel {
    Orgnization = 0,
    Department = 1,
    Person = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ComposeType {
    Star = 0,
    Mind = 1,
    UpDown = 2,
    DownUp = 3,
    LeftRight = 4,
    RightLeft = 5,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewRoadmapRequest {
    #[prost(string, tag = "1")]
    pub tree_id: std::string::String,
    #[prost(string, tag = "2")]
    pub owner_id: std::string::String,
    #[prost(bytes, tag = "3")]
    pub name: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewRoadmapResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameRoadmapRequest {
    #[prost(string, tag = "1")]
    pub tree_id: std::string::String,
    #[prost(string, tag = "2")]
    pub owner_id: std::string::String,
    #[prost(bytes, tag = "3")]
    pub name: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameRoadmapResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefrenceRoadmapRequest {
    #[prost(string, tag = "1")]
    pub tree_id: std::string::String,
    #[prost(string, tag = "2")]
    pub owner_id: std::string::String,
    #[prost(bytes, tag = "3")]
    pub name: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefrenceRoadmapResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewQuestionRequest {
    #[prost(string, tag = "1")]
    pub tree_id: std::string::String,
    #[prost(string, tag = "2")]
    pub owner_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewQuestionResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditQuestionRequest {
    #[prost(string, tag = "1")]
    pub tree_id: std::string::String,
    #[prost(string, tag = "2")]
    pub owner_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditQuestionResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveQuestionRequest {
    #[prost(string, tag = "1")]
    pub tree_id: std::string::String,
    #[prost(string, tag = "2")]
    pub owner_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveQuestionResponse {
    #[prost(string, tag = "1")]
    pub result: std::string::String,
}
