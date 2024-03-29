/// ping 网络是否正常， 双向流
/// 发送一个时间到服务端，判断是否正常和时间间隔
/// 第一次发送0，之后返回接收到的时间
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingRequest {
    /// 编号
    #[prost(uint64, tag = "1")]
    pub index: u64,
    /// 设备id， 用于区分不同设备端，根据情况需要设置，比如使用帐号作为id
    #[prost(string, tag = "2")]
    pub device_id: ::prost::alloc::string::String,
    /// 第一次发送0， 第二次返回服务器时间
    #[prost(uint64, tag = "3")]
    pub time: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingResponse {
    /// 返回ping请求的时间
    #[prost(uint64, tag = "1")]
    pub index: u64,
    #[prost(uint64, tag = "2")]
    pub time: u64,
}
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
/// 新国家编码
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewCountryCodeRequest {
    #[prost(message, optional, tag = "1")]
    pub name: ::core::option::Option<Name>,
    #[prost(string, tag = "3")]
    pub code: ::prost::alloc::string::String,
    /// 母语名
    #[prost(string, tag = "2")]
    pub native: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub phone_area_code: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "5")]
    pub languages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewCountryCodeResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 取得编码列表, 读取不需要权限
/// 客户端应该缓存这个列表
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCountryCodesRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCountryCodesResponse {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub country_codes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
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
/// 取得编码列表, 读取不需要权限
/// 客户端应该缓存这个列表
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLanguageCodesRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLanguageCodesResponse {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub language_codes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// TODO: 可能不需要
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateLanguageCodeRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub new_code: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub new_native: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateLanguageCodeResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
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
/// 新区号编码
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewPhoneAreaCodeRequest {
    #[prost(message, optional, tag = "1")]
    pub name: ::core::option::Option<Name>,
    #[prost(string, tag = "2")]
    pub code: ::prost::alloc::string::String,
    /// 使用地区
    #[prost(string, repeated, tag = "3")]
    pub areas: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewPhoneAreaCodeResponse {
    /// 成功返回新区号编码
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 取得区号编码列表, 读取不需要权限
/// 客户端应该缓存这个列表
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPhoneAreaCodesRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPhoneAreaCodesResponse {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub phone_area_codes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 通用常量不需要参数的取得接口，简化api
/// 如果常量需要参数，则需要单独定义接口
/// 每种可能有提供有自己的访问接口
/// 常量一般不需要权限控制
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConstantsRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConstantsResponse {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub constants: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
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
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FieldDataType {
    /// protobuf 类型
    FieldDataytypeUnknown = 0,
    FieldDatatypeDouble = 1,
    FieldDatatypeFloat = 2,
    FieldDatatypeInt32 = 3,
    FieldDatatypeInt64 = 4,
    FieldDatatypeUint32 = 5,
    FieldDatatypeUint64 = 6,
    FieldDatatypeSint32 = 7,
    FieldDatatypeSint64 = 8,
    FieldDatatypeFixed32 = 9,
    FieldDatatypeFixed64 = 10,
    FieldDatatypeSfixed32 = 11,
    FieldDatatypeSfixed64 = 12,
    FieldDatatypeBool = 13,
    FieldDatatypeString = 14,
    FieldDatatypeBytes = 15,
    /// 自定义类型
    /// 范围
    FieldDatatypeRange = 16,
    /// vec2
    FieldDatatypeVec2 = 17,
    /// vec3
    FieldDatatypeVec3 = 18,
    /// vec4
    FieldDatatypeVec4 = 19,
    /// 时间
    FieldDatatypeTime = 20,
}
impl FieldDataType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FieldDataType::FieldDataytypeUnknown => "FIELD_DATAYTYPE_UNKNOWN",
            FieldDataType::FieldDatatypeDouble => "FIELD_DATATYPE_DOUBLE",
            FieldDataType::FieldDatatypeFloat => "FIELD_DATATYPE_FLOAT",
            FieldDataType::FieldDatatypeInt32 => "FIELD_DATATYPE_INT32",
            FieldDataType::FieldDatatypeInt64 => "FIELD_DATATYPE_INT64",
            FieldDataType::FieldDatatypeUint32 => "FIELD_DATATYPE_UINT32",
            FieldDataType::FieldDatatypeUint64 => "FIELD_DATATYPE_UINT64",
            FieldDataType::FieldDatatypeSint32 => "FIELD_DATATYPE_SINT32",
            FieldDataType::FieldDatatypeSint64 => "FIELD_DATATYPE_SINT64",
            FieldDataType::FieldDatatypeFixed32 => "FIELD_DATATYPE_FIXED32",
            FieldDataType::FieldDatatypeFixed64 => "FIELD_DATATYPE_FIXED64",
            FieldDataType::FieldDatatypeSfixed32 => "FIELD_DATATYPE_SFIXED32",
            FieldDataType::FieldDatatypeSfixed64 => "FIELD_DATATYPE_SFIXED64",
            FieldDataType::FieldDatatypeBool => "FIELD_DATATYPE_BOOL",
            FieldDataType::FieldDatatypeString => "FIELD_DATATYPE_STRING",
            FieldDataType::FieldDatatypeBytes => "FIELD_DATATYPE_BYTES",
            FieldDataType::FieldDatatypeRange => "FIELD_DATATYPE_RANGE",
            FieldDataType::FieldDatatypeVec2 => "FIELD_DATATYPE_VEC2",
            FieldDataType::FieldDatatypeVec3 => "FIELD_DATATYPE_VEC3",
            FieldDataType::FieldDatatypeVec4 => "FIELD_DATATYPE_VEC4",
            FieldDataType::FieldDatatypeTime => "FIELD_DATATYPE_TIME",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FIELD_DATAYTYPE_UNKNOWN" => Some(Self::FieldDataytypeUnknown),
            "FIELD_DATATYPE_DOUBLE" => Some(Self::FieldDatatypeDouble),
            "FIELD_DATATYPE_FLOAT" => Some(Self::FieldDatatypeFloat),
            "FIELD_DATATYPE_INT32" => Some(Self::FieldDatatypeInt32),
            "FIELD_DATATYPE_INT64" => Some(Self::FieldDatatypeInt64),
            "FIELD_DATATYPE_UINT32" => Some(Self::FieldDatatypeUint32),
            "FIELD_DATATYPE_UINT64" => Some(Self::FieldDatatypeUint64),
            "FIELD_DATATYPE_SINT32" => Some(Self::FieldDatatypeSint32),
            "FIELD_DATATYPE_SINT64" => Some(Self::FieldDatatypeSint64),
            "FIELD_DATATYPE_FIXED32" => Some(Self::FieldDatatypeFixed32),
            "FIELD_DATATYPE_FIXED64" => Some(Self::FieldDatatypeFixed64),
            "FIELD_DATATYPE_SFIXED32" => Some(Self::FieldDatatypeSfixed32),
            "FIELD_DATATYPE_SFIXED64" => Some(Self::FieldDatatypeSfixed64),
            "FIELD_DATATYPE_BOOL" => Some(Self::FieldDatatypeBool),
            "FIELD_DATATYPE_STRING" => Some(Self::FieldDatatypeString),
            "FIELD_DATATYPE_BYTES" => Some(Self::FieldDatatypeBytes),
            "FIELD_DATATYPE_RANGE" => Some(Self::FieldDatatypeRange),
            "FIELD_DATATYPE_VEC2" => Some(Self::FieldDatatypeVec2),
            "FIELD_DATATYPE_VEC3" => Some(Self::FieldDatatypeVec3),
            "FIELD_DATATYPE_VEC4" => Some(Self::FieldDatatypeVec4),
            "FIELD_DATATYPE_TIME" => Some(Self::FieldDatatypeTime),
            _ => None,
        }
    }
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
pub struct ChangeEntityOwnerRequest {
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
pub struct ChangeEntityOwnerResponse {
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
/// 依据id取得单个实体
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
/// 依据id列表取得多个实体
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEntitiesRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    /// 列表最长100, 根据需要指定长度
    #[prost(string, repeated, tag = "2")]
    pub entity_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEntitiesResponse {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub entities: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 依据页码取得实体页列表，页码从1开始
/// 需要先取得实体总数，然后计算页数
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
/// ---------
/// 更新检查
/// ---------
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityTimestamp {
    #[prost(string, tag = "1")]
    pub entity_id: ::prost::alloc::string::String,
    /// 格式二进制 bson Document 形式{"value": Timestamp()}
    #[prost(bytes = "vec", tag = "2")]
    pub timestamp: ::prost::alloc::vec::Vec<u8>,
}
/// 检查实体是否有更新，返回有更新的实体
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckEntitiesUpdateRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    /// 列表最长不能超过100
    #[prost(message, repeated, tag = "2")]
    pub entities: ::prost::alloc::vec::Vec<EntityTimestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckEntitiesUpdateResponse {
    /// 如果有则返回bson新实体，否则返回空
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub entities: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 检查迟于指定时间是否有更新
/// 返回编号列表页流
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckUpdatesLaterThenTimeRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    /// 格式二进制 bson Document 形式{"value": Timestamp()}
    #[prost(bytes = "vec", tag = "2")]
    pub timestamp: ::prost::alloc::vec::Vec<u8>,
    /// 是否按时间升序排列, 默认降序
    #[prost(bool, tag = "3")]
    pub ascending_order: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckUpdatesLaterThenTimeResponse {
    /// 分组返回，每组最多20条
    /// 最多返回1000条
    #[prost(string, repeated, tag = "1")]
    pub entity_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewCategoryRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
    #[prost(message, optional, tag = "2")]
    pub name: ::core::option::Option<Name>,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewCategoryResponse {
    /// 成功返回id, 失败返回错误信息
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCategoriesRequest {
    #[prost(int32, tag = "1")]
    pub manage_id: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCategoriesResponse {
    /// bson bytes
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub categories: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 标记实体到类, 将品类编号添加到实体的品类列表中
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkEntityCategoriesRequest {
    #[prost(int32, tag = "2")]
    pub manage_id: i32,
    #[prost(string, tag = "3")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "4")]
    pub category_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkEntityCategoriesResponse {
    /// 成功返回“ok”, 失败返回错误信息
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 取消标记品类，将品类从实体品类列表中删除
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnmarkEntityCategoriesRequest {
    #[prost(int32, tag = "2")]
    pub manage_id: i32,
    #[prost(string, tag = "3")]
    pub entity_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "4")]
    pub category_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnmarkEntityCategoriesResponse {
    /// 成功返回“ok”, 失败返回错误信息
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewTagRequest {
    #[prost(int32, tag = "1")]
    pub target_manage_id: i32,
    #[prost(message, optional, tag = "2")]
    pub name: ::core::option::Option<Name>,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewTagResponse {
    /// 成功返回id, 失败返回错误信息
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 添加标签到某个实体
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddTagsToEntityRequest {
    #[prost(string, repeated, tag = "1")]
    pub tag_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int32, tag = "2")]
    pub target_manage_id: i32,
    #[prost(string, tag = "3")]
    pub target_entity_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddTagsToEntityResponse {
    /// 成功返回“ok”, 失败返回错误信息
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTagsRequest {
    #[prost(int32, tag = "1")]
    pub target_manage_id: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTagsResponse {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// 移除标签
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveTagsFromEntityRequest {
    #[prost(int32, tag = "1")]
    pub target_manage_id: i32,
    #[prost(string, tag = "2")]
    pub target_entity_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub tag_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveTagsFromEntityResponse {
    /// 成功返回“ok”, 失败返回错误信息
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
