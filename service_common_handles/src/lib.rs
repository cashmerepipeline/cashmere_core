/*
Author: 闫刚 (yes7rose@sina.com)
lib.rs (c) 2021
Desc: 服务处理定义
Created:  2021-02-14T09:31:29.747Z
Modified: !date!
*/

#[macro_use]
extern crate log;
extern crate core;

pub mod area_service_handles;
pub mod name_handle_service;
pub mod name_utils;
pub mod manage_service_handle;
pub mod data_service_handles;
pub mod entity_service_handles;

use std::pin::Pin;
use futures::Stream;
use tonic::{Request, Response, Status, Streaming};

pub type UnaryResponseResult<T> = Result<Response<T>, Status>;
pub type StreamResponseResult<T> = Result<Response<T>, Status>;
pub type RequestStream<T> = Request<Streaming<T>>;
pub type ResponseStream<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send>>;


use manage_define::cashmere::AreaLevel;
use manage_define::cashmere::SlotType;
