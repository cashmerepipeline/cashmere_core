use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum FieldDataType {
    None,
    Bool,
    Int32,
    Int64,
    UInt32,
    UInt64,
    F32,
    F64,
    Range,
    String,
    List,
    Bytes,
    Map,
    Date,
    DateTime,
}

impl From<String> for FieldDataType {
    fn from(s: String) -> Self {
        if s == *"Bool" {
            FieldDataType::Bool
        } else if s == *"Int32" {
            FieldDataType::Int32
        } else if s == *"Int64" {
            FieldDataType::Int64
        } else if s == *"UInt32" {
            FieldDataType::UInt32
        } else if s == *"UInt64" {
            FieldDataType::UInt64
        } else if s == *"F32" {
            FieldDataType::F32
        } else if s == *"F64" {
            FieldDataType::F64
        } else if s == *"Range" {
            FieldDataType::Range
        } else if s == *"String" {
            FieldDataType::String
        } else if s == *"List" {
            FieldDataType::List
        } else if s == *"Bytes" {
            FieldDataType::Bytes
        } else if s == *"Map" {
            FieldDataType::Map
        } else if s == *"Date" {
            FieldDataType::Date
        } else if s == *"DateTime" {
            FieldDataType::DateTime
        } else {
            FieldDataType::None
        }
    }
}
