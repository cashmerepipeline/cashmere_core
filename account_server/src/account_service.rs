/// 添加账号, 需要手工添加账号的情景
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAccountRequest {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub department_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub group_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub phone: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAccountResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 注册账号, 用户需要自己注册账号的情景
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterRequest {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub department_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub phone: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
/// 自己修改手机号码
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeOwnPhoneRequest {
    #[prost(string, tag="1")]
    pub old_phone: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub new_phone: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangePhoneOwnResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
