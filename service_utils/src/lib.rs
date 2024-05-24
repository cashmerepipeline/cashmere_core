use dependencies_sync::rust_i18n::{self, i18n};
i18n!("locales");

pub mod types;

pub use get_constants::*;
pub use send_stream_error::*;
pub use hard_codes_utils::*;
pub use send_stream_response::*;

mod get_constants;
mod hard_codes_utils;
mod send_stream_error;
mod send_stream_response;