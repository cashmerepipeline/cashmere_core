use futures::Stream;
use std::pin::Pin;
use tonic::{Request, Response, Status, Streaming};

pub type UnaryResponseResult<T> = Result<Response<T>, Status>;
pub type RequestStream<T> = Request<Streaming<T>>;
pub type ResponseStream<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send>>;
pub type StreamResponseResult<T> = Result<Response<ResponseStream<T>>, Status>;
