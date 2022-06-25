use std::pin::Pin;

use futures::Stream;
use tonic::{Request, Response, Status, Streaming};

pub type UnaryResponseResult<T> = Result<Response<T>, Status>;
pub type StreamResponseResult<T> = Result<Response<T>, Status>;
pub type RequestStream<T> = Request<Streaming<T>>;
pub type ResponseStream<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send>>;
