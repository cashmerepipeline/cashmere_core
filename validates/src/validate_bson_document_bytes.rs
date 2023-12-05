use dependencies_sync::{
    bson::{self, Document},
    rust_i18n::{self, t},
    tonic::Status,
};

pub fn validate_bson_document_bytes(bson_bytes: &Vec<u8>) -> Result<(), Status> {
    if let Err(err) = bson::from_slice::<Document>(bson_bytes.as_slice()) {
        return Err(Status::invalid_argument(format!(
            "{}: {}",
            t!("无效bson document格式数据"),
            err
        )));
    }
    Ok(())
}
