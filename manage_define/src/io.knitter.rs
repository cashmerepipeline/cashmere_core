#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSpecsRequest {
    #[prost(int32, tag="1")]
    pub owner_manage_id: i32,
    #[prost(string, tag="2")]
    pub owner_entity_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub name: ::core::option::Option<super::super::cashmere::Name>,
    #[prost(string, tag="4")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewSpecsResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSpecsRequest {
    #[prost(int32, tag="1")]
    pub owner_manage_id: i32,
    #[prost(string, tag="2")]
    pub owner_entity_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSpecsResponse {
    #[prost(bytes="vec", repeated, tag="1")]
    pub specses: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSpecsPrefabsRequest {
    #[prost(string, tag="1")]
    pub specs_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSpecsPrefabsResponse {
    #[prost(bytes="vec", repeated, tag="1")]
    pub prefabs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewPrefabRequest {
    #[prost(string, tag="1")]
    pub specs_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub name: ::core::option::Option<super::super::cashmere::Name>,
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewPrefabResponse {
    #[prost(string, tag="1")]
    pub result: ::prost::alloc::string::String,
}
