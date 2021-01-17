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
#[doc = r" Generated client implementations."]
pub mod cashmere_grpc_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct CashmereGrpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CashmereGrpcClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> CashmereGrpcClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " =====管理====="]
        #[doc = " 取得管理列表"]
        pub async fn get_manages(
            &mut self,
            request: impl tonic::IntoRequest<super::GetManagesRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::Entity>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/swb.cashmere.CashmereGrpc/GetManages");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " =====管理描写====="]
        pub async fn get_manage_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::GetManageSchemaRequest>,
        ) -> Result<tonic::Response<super::GetManageSchemaResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/swb.cashmere.CashmereGrpc/GetManageSchema");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn new_schema_field(
            &mut self,
            request: impl tonic::IntoRequest<super::NewSchemaFieldRequest>,
        ) -> Result<tonic::Response<super::NewSchemaFieldResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/swb.cashmere.CashmereGrpc/NewSchemaField");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 编辑属性的用处？？？编号和类型 不可变，只能修改名字"]
        pub async fn edit_schema_field_name(
            &mut self,
            request: impl tonic::IntoRequest<super::EditSchemaFieldNameRequest>,
        ) -> Result<tonic::Response<super::EditSchemaFieldNameResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/swb.cashmere.CashmereGrpc/EditSchemaFieldName",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 设置属性 移除标记 为真"]
        pub async fn remove_schema_field(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveSchemaFieldRequest>,
        ) -> Result<tonic::Response<super::RemoveSchemaFieldResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/swb.cashmere.CashmereGrpc/RemoveSchemaField",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " =====管理对象====="]
        pub async fn new_manage_entity(
            &mut self,
            request: impl tonic::IntoRequest<super::NewEntityRequest>,
        ) -> Result<tonic::Response<super::NewEntityResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/swb.cashmere.CashmereGrpc/NewManageEntity");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn edit_manage_entity(
            &mut self,
            request: impl tonic::IntoRequest<super::EditEntityRequest>,
        ) -> Result<tonic::Response<super::EditEntityResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/swb.cashmere.CashmereGrpc/EditManageEntity");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_manage_entity(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveEntityRequest>,
        ) -> Result<tonic::Response<super::RemoveEntityResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/swb.cashmere.CashmereGrpc/RemoveManageEntity",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " =====管理对象模板====="]
        pub async fn new_entity_template(
            &mut self,
            request: impl tonic::IntoRequest<super::NewEntityTemplateRequest>,
        ) -> Result<tonic::Response<super::NewEntityTemplateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/swb.cashmere.CashmereGrpc/NewEntityTemplate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn edit_entity_template(
            &mut self,
            request: impl tonic::IntoRequest<super::EditEntityTemplateRequest>,
        ) -> Result<tonic::Response<super::EditEntityTemplateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/swb.cashmere.CashmereGrpc/EditEntityTemplate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_entity_template(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveEntityTemplateRequest>,
        ) -> Result<tonic::Response<super::RemoveEntityTemplateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/swb.cashmere.CashmereGrpc/RemoveEntityTemplate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " =====管理映像====="]
        pub async fn get_manage_view(
            &mut self,
            request: impl tonic::IntoRequest<super::GetManageViewRequest>,
        ) -> Result<tonic::Response<super::GetManageViewResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/swb.cashmere.CashmereGrpc/GetManageView");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " =====事件====="]
        #[doc = " 事件管理"]
        pub async fn new_event(
            &mut self,
            request: impl tonic::IntoRequest<super::NewEventRequest>,
        ) -> Result<tonic::Response<super::NewEventResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/swb.cashmere.CashmereGrpc/NewEvent");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn new_event_queue(
            &mut self,
            request: impl tonic::IntoRequest<super::NewEventQueueRequest>,
        ) -> Result<tonic::Response<super::NewEventQueueResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/swb.cashmere.CashmereGrpc/NewEventQueue");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn new_event_handle(
            &mut self,
            request: impl tonic::IntoRequest<super::NewEventHandleRequest>,
        ) -> Result<tonic::Response<super::NewEventHandleResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/swb.cashmere.CashmereGrpc/NewEventHandle");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " =====工作====="]
        #[doc = " 工作管理"]
        pub async fn new_work(
            &mut self,
            request: impl tonic::IntoRequest<super::NewWorkRequest>,
        ) -> Result<tonic::Response<super::NewWorkResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/swb.cashmere.CashmereGrpc/NewWork");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn new_phase_for_work(
            &mut self,
            request: impl tonic::IntoRequest<super::NewPhaseForWorkRequest>,
        ) -> Result<tonic::Response<super::NewPhaseForWorkResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/swb.cashmere.CashmereGrpc/NewPhaseForWork");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn new_work_node_for_procedure_graph(
            &mut self,
            request: impl tonic::IntoRequest<super::NewWorkNodeForProcedureGraphRequest>,
        ) -> Result<tonic::Response<super::NewWorkNodeForProcedureGraphResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/swb.cashmere.CashmereGrpc/NewWorkNodeForProcedureGraph",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 指派工作"]
        pub async fn assign_work_node_to_worker(
            &mut self,
            request: impl tonic::IntoRequest<super::AssignWorkNodeToWorkerRequest>,
        ) -> Result<tonic::Response<super::AssignWorkNodeToWorkerResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/swb.cashmere.CashmereGrpc/AssignWorkNodeToWorker",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 工作节点"]
        pub async fn create_work_node_link(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWorkNodeLinkRequest>,
        ) -> Result<tonic::Response<super::CreateWorkNodeLinkResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/swb.cashmere.CashmereGrpc/CreateWorkNodeLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_work_node_link(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveWorkNodeLinkRequest>,
        ) -> Result<tonic::Response<super::RemoveWorkNodeLinkResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/swb.cashmere.CashmereGrpc/RemoveWorkNodeLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn new_data_slot_for_work_node(
            &mut self,
            request: impl tonic::IntoRequest<super::NewDataSlotForWorkNodeRequest>,
        ) -> Result<tonic::Response<super::NewDataSlotForWorkNodeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/swb.cashmere.CashmereGrpc/NewDataSlotForWorkNode",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 新任务"]
        pub async fn new_task_for_work_node(
            &mut self,
            request: impl tonic::IntoRequest<super::NewTaskForWorkNodeRequest>,
        ) -> Result<tonic::Response<super::NewTaskForWorkNodeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/swb.cashmere.CashmereGrpc/NewTaskForWorkNode",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 标记任务状态"]
        pub async fn mark_task_status(
            &mut self,
            request: impl tonic::IntoRequest<super::MarkTaskStatusRequest>,
        ) -> Result<tonic::Response<super::MarkTaskStatusResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/swb.cashmere.CashmereGrpc/MarkTaskStatus");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " 提交任务"]
        pub async fn commit_task(
            &mut self,
            request: impl tonic::IntoRequest<super::CommitTaskRequest>,
        ) -> Result<tonic::Response<super::CommitTaskResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/swb.cashmere.CashmereGrpc/CommitTask");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " =====数据====="]
        pub async fn new_data_for_task(
            &mut self,
            request: impl tonic::IntoRequest<super::NewDataForTaskRequest>,
        ) -> Result<tonic::Response<super::NewDataForTaskResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/swb.cashmere.CashmereGrpc/NewDataForTask");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn associate_data_to_task(
            &mut self,
            request: impl tonic::IntoRequest<super::AssociateDataToTaskRequest>,
        ) -> Result<tonic::Response<super::AssociateDataToTaskResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/swb.cashmere.CashmereGrpc/AssociateDataToTask",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CashmereGrpcClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CashmereGrpcClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CashmereGrpcClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod cashmere_grpc_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with CashmereGrpcServer."]
    #[async_trait]
    pub trait CashmereGrpc: Send + Sync + 'static {
        #[doc = "Server streaming response type for the GetManages method."]
        type GetManagesStream: Stream<Item = Result<super::Entity, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " =====管理====="]
        #[doc = " 取得管理列表"]
        async fn get_manages(
            &self,
            request: tonic::Request<super::GetManagesRequest>,
        ) -> Result<tonic::Response<Self::GetManagesStream>, tonic::Status>;
        #[doc = " =====管理描写====="]
        async fn get_manage_schema(
            &self,
            request: tonic::Request<super::GetManageSchemaRequest>,
        ) -> Result<tonic::Response<super::GetManageSchemaResponse>, tonic::Status>;
        async fn new_schema_field(
            &self,
            request: tonic::Request<super::NewSchemaFieldRequest>,
        ) -> Result<tonic::Response<super::NewSchemaFieldResponse>, tonic::Status>;
        #[doc = " 编辑属性的用处？？？编号和类型 不可变，只能修改名字"]
        async fn edit_schema_field_name(
            &self,
            request: tonic::Request<super::EditSchemaFieldNameRequest>,
        ) -> Result<tonic::Response<super::EditSchemaFieldNameResponse>, tonic::Status>;
        #[doc = " 设置属性 移除标记 为真"]
        async fn remove_schema_field(
            &self,
            request: tonic::Request<super::RemoveSchemaFieldRequest>,
        ) -> Result<tonic::Response<super::RemoveSchemaFieldResponse>, tonic::Status>;
        #[doc = " =====管理对象====="]
        async fn new_manage_entity(
            &self,
            request: tonic::Request<super::NewEntityRequest>,
        ) -> Result<tonic::Response<super::NewEntityResponse>, tonic::Status>;
        async fn edit_manage_entity(
            &self,
            request: tonic::Request<super::EditEntityRequest>,
        ) -> Result<tonic::Response<super::EditEntityResponse>, tonic::Status>;
        async fn remove_manage_entity(
            &self,
            request: tonic::Request<super::RemoveEntityRequest>,
        ) -> Result<tonic::Response<super::RemoveEntityResponse>, tonic::Status>;
        #[doc = " =====管理对象模板====="]
        async fn new_entity_template(
            &self,
            request: tonic::Request<super::NewEntityTemplateRequest>,
        ) -> Result<tonic::Response<super::NewEntityTemplateResponse>, tonic::Status>;
        async fn edit_entity_template(
            &self,
            request: tonic::Request<super::EditEntityTemplateRequest>,
        ) -> Result<tonic::Response<super::EditEntityTemplateResponse>, tonic::Status>;
        async fn remove_entity_template(
            &self,
            request: tonic::Request<super::RemoveEntityTemplateRequest>,
        ) -> Result<tonic::Response<super::RemoveEntityTemplateResponse>, tonic::Status>;
        #[doc = " =====管理映像====="]
        async fn get_manage_view(
            &self,
            request: tonic::Request<super::GetManageViewRequest>,
        ) -> Result<tonic::Response<super::GetManageViewResponse>, tonic::Status>;
        #[doc = " =====事件====="]
        #[doc = " 事件管理"]
        async fn new_event(
            &self,
            request: tonic::Request<super::NewEventRequest>,
        ) -> Result<tonic::Response<super::NewEventResponse>, tonic::Status>;
        async fn new_event_queue(
            &self,
            request: tonic::Request<super::NewEventQueueRequest>,
        ) -> Result<tonic::Response<super::NewEventQueueResponse>, tonic::Status>;
        async fn new_event_handle(
            &self,
            request: tonic::Request<super::NewEventHandleRequest>,
        ) -> Result<tonic::Response<super::NewEventHandleResponse>, tonic::Status>;
        #[doc = " =====工作====="]
        #[doc = " 工作管理"]
        async fn new_work(
            &self,
            request: tonic::Request<super::NewWorkRequest>,
        ) -> Result<tonic::Response<super::NewWorkResponse>, tonic::Status>;
        async fn new_phase_for_work(
            &self,
            request: tonic::Request<super::NewPhaseForWorkRequest>,
        ) -> Result<tonic::Response<super::NewPhaseForWorkResponse>, tonic::Status>;
        async fn new_work_node_for_procedure_graph(
            &self,
            request: tonic::Request<super::NewWorkNodeForProcedureGraphRequest>,
        ) -> Result<tonic::Response<super::NewWorkNodeForProcedureGraphResponse>, tonic::Status>;
        #[doc = " 指派工作"]
        async fn assign_work_node_to_worker(
            &self,
            request: tonic::Request<super::AssignWorkNodeToWorkerRequest>,
        ) -> Result<tonic::Response<super::AssignWorkNodeToWorkerResponse>, tonic::Status>;
        #[doc = " 工作节点"]
        async fn create_work_node_link(
            &self,
            request: tonic::Request<super::CreateWorkNodeLinkRequest>,
        ) -> Result<tonic::Response<super::CreateWorkNodeLinkResponse>, tonic::Status>;
        async fn remove_work_node_link(
            &self,
            request: tonic::Request<super::RemoveWorkNodeLinkRequest>,
        ) -> Result<tonic::Response<super::RemoveWorkNodeLinkResponse>, tonic::Status>;
        async fn new_data_slot_for_work_node(
            &self,
            request: tonic::Request<super::NewDataSlotForWorkNodeRequest>,
        ) -> Result<tonic::Response<super::NewDataSlotForWorkNodeResponse>, tonic::Status>;
        #[doc = " 新任务"]
        async fn new_task_for_work_node(
            &self,
            request: tonic::Request<super::NewTaskForWorkNodeRequest>,
        ) -> Result<tonic::Response<super::NewTaskForWorkNodeResponse>, tonic::Status>;
        #[doc = " 标记任务状态"]
        async fn mark_task_status(
            &self,
            request: tonic::Request<super::MarkTaskStatusRequest>,
        ) -> Result<tonic::Response<super::MarkTaskStatusResponse>, tonic::Status>;
        #[doc = " 提交任务"]
        async fn commit_task(
            &self,
            request: tonic::Request<super::CommitTaskRequest>,
        ) -> Result<tonic::Response<super::CommitTaskResponse>, tonic::Status>;
        #[doc = " =====数据====="]
        async fn new_data_for_task(
            &self,
            request: tonic::Request<super::NewDataForTaskRequest>,
        ) -> Result<tonic::Response<super::NewDataForTaskResponse>, tonic::Status>;
        async fn associate_data_to_task(
            &self,
            request: tonic::Request<super::AssociateDataToTaskRequest>,
        ) -> Result<tonic::Response<super::AssociateDataToTaskResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct CashmereGrpcServer<T: CashmereGrpc> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: CashmereGrpc> CashmereGrpcServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for CashmereGrpcServer<T>
    where
        T: CashmereGrpc,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/swb.cashmere.CashmereGrpc/GetManages" => {
                    #[allow(non_camel_case_types)]
                    struct GetManagesSvc<T: CashmereGrpc>(pub Arc<T>);
                    impl<T: CashmereGrpc>
                        tonic::server::ServerStreamingService<super::GetManagesRequest>
                        for GetManagesSvc<T>
                    {
                        type Response = super::Entity;
                        type ResponseStream = T::GetManagesStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetManagesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_manages(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = GetManagesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/swb.cashmere.CashmereGrpc/GetManageSchema" => {
                    #[allow(non_camel_case_types)]
                    struct GetManageSchemaSvc<T: CashmereGrpc>(pub Arc<T>);
                    impl<T: CashmereGrpc> tonic::server::UnaryService<super::GetManageSchemaRequest>
                        for GetManageSchemaSvc<T>
                    {
                        type Response = super::GetManageSchemaResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetManageSchemaRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_manage_schema(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetManageSchemaSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/swb.cashmere.CashmereGrpc/NewSchemaField" => {
                    #[allow(non_camel_case_types)]
                    struct NewSchemaFieldSvc<T: CashmereGrpc>(pub Arc<T>);
                    impl<T: CashmereGrpc> tonic::server::UnaryService<super::NewSchemaFieldRequest>
                        for NewSchemaFieldSvc<T>
                    {
                        type Response = super::NewSchemaFieldResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewSchemaFieldRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_schema_field(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = NewSchemaFieldSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/swb.cashmere.CashmereGrpc/EditSchemaFieldName" => {
                    #[allow(non_camel_case_types)]
                    struct EditSchemaFieldNameSvc<T: CashmereGrpc>(pub Arc<T>);
                    impl<T: CashmereGrpc>
                        tonic::server::UnaryService<super::EditSchemaFieldNameRequest>
                        for EditSchemaFieldNameSvc<T>
                    {
                        type Response = super::EditSchemaFieldNameResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EditSchemaFieldNameRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).edit_schema_field_name(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = EditSchemaFieldNameSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/swb.cashmere.CashmereGrpc/RemoveSchemaField" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveSchemaFieldSvc<T: CashmereGrpc>(pub Arc<T>);
                    impl<T: CashmereGrpc>
                        tonic::server::UnaryService<super::RemoveSchemaFieldRequest>
                        for RemoveSchemaFieldSvc<T>
                    {
                        type Response = super::RemoveSchemaFieldResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveSchemaFieldRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_schema_field(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RemoveSchemaFieldSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/swb.cashmere.CashmereGrpc/NewManageEntity" => {
                    #[allow(non_camel_case_types)]
                    struct NewManageEntitySvc<T: CashmereGrpc>(pub Arc<T>);
                    impl<T: CashmereGrpc> tonic::server::UnaryService<super::NewEntityRequest>
                        for NewManageEntitySvc<T>
                    {
                        type Response = super::NewEntityResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewEntityRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_manage_entity(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = NewManageEntitySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/swb.cashmere.CashmereGrpc/EditManageEntity" => {
                    #[allow(non_camel_case_types)]
                    struct EditManageEntitySvc<T: CashmereGrpc>(pub Arc<T>);
                    impl<T: CashmereGrpc> tonic::server::UnaryService<super::EditEntityRequest>
                        for EditManageEntitySvc<T>
                    {
                        type Response = super::EditEntityResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EditEntityRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).edit_manage_entity(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = EditManageEntitySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/swb.cashmere.CashmereGrpc/RemoveManageEntity" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveManageEntitySvc<T: CashmereGrpc>(pub Arc<T>);
                    impl<T: CashmereGrpc> tonic::server::UnaryService<super::RemoveEntityRequest>
                        for RemoveManageEntitySvc<T>
                    {
                        type Response = super::RemoveEntityResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveEntityRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_manage_entity(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RemoveManageEntitySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/swb.cashmere.CashmereGrpc/NewEntityTemplate" => {
                    #[allow(non_camel_case_types)]
                    struct NewEntityTemplateSvc<T: CashmereGrpc>(pub Arc<T>);
                    impl<T: CashmereGrpc>
                        tonic::server::UnaryService<super::NewEntityTemplateRequest>
                        for NewEntityTemplateSvc<T>
                    {
                        type Response = super::NewEntityTemplateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewEntityTemplateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_entity_template(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = NewEntityTemplateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/swb.cashmere.CashmereGrpc/EditEntityTemplate" => {
                    #[allow(non_camel_case_types)]
                    struct EditEntityTemplateSvc<T: CashmereGrpc>(pub Arc<T>);
                    impl<T: CashmereGrpc>
                        tonic::server::UnaryService<super::EditEntityTemplateRequest>
                        for EditEntityTemplateSvc<T>
                    {
                        type Response = super::EditEntityTemplateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EditEntityTemplateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).edit_entity_template(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = EditEntityTemplateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/swb.cashmere.CashmereGrpc/RemoveEntityTemplate" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveEntityTemplateSvc<T: CashmereGrpc>(pub Arc<T>);
                    impl<T: CashmereGrpc>
                        tonic::server::UnaryService<super::RemoveEntityTemplateRequest>
                        for RemoveEntityTemplateSvc<T>
                    {
                        type Response = super::RemoveEntityTemplateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveEntityTemplateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_entity_template(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RemoveEntityTemplateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/swb.cashmere.CashmereGrpc/GetManageView" => {
                    #[allow(non_camel_case_types)]
                    struct GetManageViewSvc<T: CashmereGrpc>(pub Arc<T>);
                    impl<T: CashmereGrpc> tonic::server::UnaryService<super::GetManageViewRequest>
                        for GetManageViewSvc<T>
                    {
                        type Response = super::GetManageViewResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetManageViewRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_manage_view(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetManageViewSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/swb.cashmere.CashmereGrpc/NewEvent" => {
                    #[allow(non_camel_case_types)]
                    struct NewEventSvc<T: CashmereGrpc>(pub Arc<T>);
                    impl<T: CashmereGrpc> tonic::server::UnaryService<super::NewEventRequest> for NewEventSvc<T> {
                        type Response = super::NewEventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewEventRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_event(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = NewEventSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/swb.cashmere.CashmereGrpc/NewEventQueue" => {
                    #[allow(non_camel_case_types)]
                    struct NewEventQueueSvc<T: CashmereGrpc>(pub Arc<T>);
                    impl<T: CashmereGrpc> tonic::server::UnaryService<super::NewEventQueueRequest>
                        for NewEventQueueSvc<T>
                    {
                        type Response = super::NewEventQueueResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewEventQueueRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_event_queue(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = NewEventQueueSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/swb.cashmere.CashmereGrpc/NewEventHandle" => {
                    #[allow(non_camel_case_types)]
                    struct NewEventHandleSvc<T: CashmereGrpc>(pub Arc<T>);
                    impl<T: CashmereGrpc> tonic::server::UnaryService<super::NewEventHandleRequest>
                        for NewEventHandleSvc<T>
                    {
                        type Response = super::NewEventHandleResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewEventHandleRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_event_handle(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = NewEventHandleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/swb.cashmere.CashmereGrpc/NewWork" => {
                    #[allow(non_camel_case_types)]
                    struct NewWorkSvc<T: CashmereGrpc>(pub Arc<T>);
                    impl<T: CashmereGrpc> tonic::server::UnaryService<super::NewWorkRequest> for NewWorkSvc<T> {
                        type Response = super::NewWorkResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewWorkRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_work(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = NewWorkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/swb.cashmere.CashmereGrpc/NewPhaseForWork" => {
                    #[allow(non_camel_case_types)]
                    struct NewPhaseForWorkSvc<T: CashmereGrpc>(pub Arc<T>);
                    impl<T: CashmereGrpc> tonic::server::UnaryService<super::NewPhaseForWorkRequest>
                        for NewPhaseForWorkSvc<T>
                    {
                        type Response = super::NewPhaseForWorkResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewPhaseForWorkRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_phase_for_work(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = NewPhaseForWorkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/swb.cashmere.CashmereGrpc/NewWorkNodeForProcedureGraph" => {
                    #[allow(non_camel_case_types)]
                    struct NewWorkNodeForProcedureGraphSvc<T: CashmereGrpc>(pub Arc<T>);
                    impl<T: CashmereGrpc>
                        tonic::server::UnaryService<super::NewWorkNodeForProcedureGraphRequest>
                        for NewWorkNodeForProcedureGraphSvc<T>
                    {
                        type Response = super::NewWorkNodeForProcedureGraphResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewWorkNodeForProcedureGraphRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).new_work_node_for_procedure_graph(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = NewWorkNodeForProcedureGraphSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/swb.cashmere.CashmereGrpc/AssignWorkNodeToWorker" => {
                    #[allow(non_camel_case_types)]
                    struct AssignWorkNodeToWorkerSvc<T: CashmereGrpc>(pub Arc<T>);
                    impl<T: CashmereGrpc>
                        tonic::server::UnaryService<super::AssignWorkNodeToWorkerRequest>
                        for AssignWorkNodeToWorkerSvc<T>
                    {
                        type Response = super::AssignWorkNodeToWorkerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AssignWorkNodeToWorkerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).assign_work_node_to_worker(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = AssignWorkNodeToWorkerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/swb.cashmere.CashmereGrpc/CreateWorkNodeLink" => {
                    #[allow(non_camel_case_types)]
                    struct CreateWorkNodeLinkSvc<T: CashmereGrpc>(pub Arc<T>);
                    impl<T: CashmereGrpc>
                        tonic::server::UnaryService<super::CreateWorkNodeLinkRequest>
                        for CreateWorkNodeLinkSvc<T>
                    {
                        type Response = super::CreateWorkNodeLinkResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateWorkNodeLinkRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_work_node_link(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateWorkNodeLinkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/swb.cashmere.CashmereGrpc/RemoveWorkNodeLink" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveWorkNodeLinkSvc<T: CashmereGrpc>(pub Arc<T>);
                    impl<T: CashmereGrpc>
                        tonic::server::UnaryService<super::RemoveWorkNodeLinkRequest>
                        for RemoveWorkNodeLinkSvc<T>
                    {
                        type Response = super::RemoveWorkNodeLinkResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveWorkNodeLinkRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_work_node_link(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RemoveWorkNodeLinkSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/swb.cashmere.CashmereGrpc/NewDataSlotForWorkNode" => {
                    #[allow(non_camel_case_types)]
                    struct NewDataSlotForWorkNodeSvc<T: CashmereGrpc>(pub Arc<T>);
                    impl<T: CashmereGrpc>
                        tonic::server::UnaryService<super::NewDataSlotForWorkNodeRequest>
                        for NewDataSlotForWorkNodeSvc<T>
                    {
                        type Response = super::NewDataSlotForWorkNodeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewDataSlotForWorkNodeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).new_data_slot_for_work_node(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = NewDataSlotForWorkNodeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/swb.cashmere.CashmereGrpc/NewTaskForWorkNode" => {
                    #[allow(non_camel_case_types)]
                    struct NewTaskForWorkNodeSvc<T: CashmereGrpc>(pub Arc<T>);
                    impl<T: CashmereGrpc>
                        tonic::server::UnaryService<super::NewTaskForWorkNodeRequest>
                        for NewTaskForWorkNodeSvc<T>
                    {
                        type Response = super::NewTaskForWorkNodeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewTaskForWorkNodeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_task_for_work_node(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = NewTaskForWorkNodeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/swb.cashmere.CashmereGrpc/MarkTaskStatus" => {
                    #[allow(non_camel_case_types)]
                    struct MarkTaskStatusSvc<T: CashmereGrpc>(pub Arc<T>);
                    impl<T: CashmereGrpc> tonic::server::UnaryService<super::MarkTaskStatusRequest>
                        for MarkTaskStatusSvc<T>
                    {
                        type Response = super::MarkTaskStatusResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MarkTaskStatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).mark_task_status(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = MarkTaskStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/swb.cashmere.CashmereGrpc/CommitTask" => {
                    #[allow(non_camel_case_types)]
                    struct CommitTaskSvc<T: CashmereGrpc>(pub Arc<T>);
                    impl<T: CashmereGrpc> tonic::server::UnaryService<super::CommitTaskRequest> for CommitTaskSvc<T> {
                        type Response = super::CommitTaskResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CommitTaskRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).commit_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CommitTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/swb.cashmere.CashmereGrpc/NewDataForTask" => {
                    #[allow(non_camel_case_types)]
                    struct NewDataForTaskSvc<T: CashmereGrpc>(pub Arc<T>);
                    impl<T: CashmereGrpc> tonic::server::UnaryService<super::NewDataForTaskRequest>
                        for NewDataForTaskSvc<T>
                    {
                        type Response = super::NewDataForTaskResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewDataForTaskRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_data_for_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = NewDataForTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/swb.cashmere.CashmereGrpc/AssociateDataToTask" => {
                    #[allow(non_camel_case_types)]
                    struct AssociateDataToTaskSvc<T: CashmereGrpc>(pub Arc<T>);
                    impl<T: CashmereGrpc>
                        tonic::server::UnaryService<super::AssociateDataToTaskRequest>
                        for AssociateDataToTaskSvc<T>
                    {
                        type Response = super::AssociateDataToTaskResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AssociateDataToTaskRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).associate_data_to_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = AssociateDataToTaskSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: CashmereGrpc> Clone for CashmereGrpcServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: CashmereGrpc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: CashmereGrpc> tonic::transport::NamedService for CashmereGrpcServer<T> {
        const NAME: &'static str = "swb.cashmere.CashmereGrpc";
    }
}
/// 登录
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginRequest {
    #[prost(string, tag = "1")]
    pub country_code: std::string::String,
    #[prost(string, tag = "2")]
    pub phone: std::string::String,
    #[prost(string, tag = "3")]
    pub password: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginResponse {
    #[prost(bytes, tag = "1")]
    pub person: std::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub token: std::string::String,
}
/// 登出
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogoutRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogoutResponse {
    #[prost(enumeration = "LoginStatus", tag = "1")]
    pub result: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LoginStatus {
    Login = 0,
    Out = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccountStatus {
    Stopped = 0,
    Actived = 1,
}
#[doc = r" Generated client implementations."]
pub mod account_grpc_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct AccountGrpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AccountGrpcClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AccountGrpcClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " 登录"]
        pub async fn login(
            &mut self,
            request: impl tonic::IntoRequest<super::LoginRequest>,
        ) -> Result<tonic::Response<super::LoginResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/swb.cashmere.AccountGrpc/Login");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AccountGrpcClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AccountGrpcClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AccountGrpcClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod account_grpc_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with AccountGrpcServer."]
    #[async_trait]
    pub trait AccountGrpc: Send + Sync + 'static {
        #[doc = " 登录"]
        async fn login(
            &self,
            request: tonic::Request<super::LoginRequest>,
        ) -> Result<tonic::Response<super::LoginResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct AccountGrpcServer<T: AccountGrpc> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: AccountGrpc> AccountGrpcServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for AccountGrpcServer<T>
    where
        T: AccountGrpc,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/swb.cashmere.AccountGrpc/Login" => {
                    #[allow(non_camel_case_types)]
                    struct LoginSvc<T: AccountGrpc>(pub Arc<T>);
                    impl<T: AccountGrpc> tonic::server::UnaryService<super::LoginRequest> for LoginSvc<T> {
                        type Response = super::LoginResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LoginRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).login(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = LoginSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: AccountGrpc> Clone for AccountGrpcServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: AccountGrpc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: AccountGrpc> tonic::transport::NamedService for AccountGrpcServer<T> {
        const NAME: &'static str = "swb.cashmere.AccountGrpc";
    }
}
