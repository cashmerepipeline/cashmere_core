/*
Author: 闫刚 (yes7rose@sina.com)
lib.rs (c) 2021
Desc: 服务处理定义
Created:  2021-02-14T09:31:29.747Z
Modified: !date!
*/

pub mod area_service_handles;
pub mod data_service_handles;
pub mod entity_service_handles;
pub mod manage_service_handle;
pub mod name_handle_service;
pub mod name_utils;

use futures::Stream;
use std::pin::Pin;
use tonic::{Request, Response, Status, Streaming};

pub type UnaryResponseResult<T> = Result<Response<T>, Status>;
pub type StreamResponseResult<T> = Result<Response<T>, Status>;
pub type RequestStream<T> = Request<Streaming<T>>;
pub type ResponseStream<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send>>;
