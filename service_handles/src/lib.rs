pub mod account_handle_service;
pub mod account_service;

pub mod cashmere;
pub mod common_handle_service;
pub mod graph_service_handle;
pub mod point_handle_service;

use tonic::{Request, Response, Status};

type UnaryResponseResult<T> = Result<Response<T>, Status>;
type StreamResponseResult<T> = Result<Response<T>, Status>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
